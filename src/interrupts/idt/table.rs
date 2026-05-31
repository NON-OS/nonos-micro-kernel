// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::{PrivilegeLevel, VirtAddr};

use super::vectors;
use crate::arch::x86_64::gdt;
use crate::interrupts::isr;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = build_idt();
}

fn build_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();

    configure_exceptions(&mut idt);
    configure_irqs(&mut idt);
    configure_syscall(&mut idt);

    idt
}

fn configure_exceptions(idt: &mut InterruptDescriptorTable) {
    idt.divide_error.set_handler_fn(isr::isr_divide_error);
    idt.debug.set_handler_fn(isr::isr_debug);

    // SAFETY: NMI uses dedicated IST stack to handle nested NMIs safely
    unsafe {
        idt.non_maskable_interrupt.set_handler_fn(isr::isr_nmi).set_stack_index(gdt::NMI_IST_INDEX);
    }

    idt.breakpoint.set_handler_fn(isr::isr_breakpoint);
    idt.overflow.set_handler_fn(isr::isr_overflow);
    idt.bound_range_exceeded.set_handler_fn(isr::isr_bound_range);
    idt.invalid_opcode.set_handler_fn(isr::isr_invalid_opcode);
    idt.device_not_available.set_handler_fn(isr::isr_device_na);

    // SAFETY: Double fault uses dedicated IST stack to recover from stack overflow
    unsafe {
        idt.double_fault
            .set_handler_addr(VirtAddr::new(isr::isr_double_fault as *const () as u64))
            .set_stack_index(gdt::DF_IST_INDEX);
    }

    idt.invalid_tss.set_handler_fn(isr::isr_invalid_tss);
    idt.segment_not_present.set_handler_fn(isr::isr_segment_not_present);
    idt.stack_segment_fault.set_handler_fn(isr::isr_stack_segment_fault);

    // SAFETY: General protection fault uses a dedicated IST stack so a
    // CPL=3 #GP cannot land on a torn TSS.RSP0 mid-context-switch.
    unsafe {
        idt.general_protection_fault
            .set_handler_fn(isr::isr_gpf)
            .set_stack_index(gdt::GP_IST_INDEX);
    }

    // SAFETY: Page fault uses the naked `page_fault_trampoline` so a #PF
    // from CPL=3 swapgs-es onto the kernel per-CPU base before the handler's
    // gs-relative accesses run (otherwise it faults on gs:0 and storms). The
    // dedicated IST stack for guard-page handling is preserved.
    unsafe {
        idt.page_fault
            .set_handler_addr(VirtAddr::new(isr::page_fault_trampoline as *const () as u64))
            .set_stack_index(gdt::PF_IST_INDEX);
    }

    idt.x87_floating_point.set_handler_fn(isr::isr_x87_fp);
    idt.alignment_check.set_handler_fn(isr::isr_alignment_check);

    // SAFETY: Machine check uses dedicated IST stack for critical hardware errors
    unsafe {
        idt.machine_check
            .set_handler_addr(VirtAddr::new(isr::isr_machine_check as *const () as u64))
            .set_stack_index(gdt::MC_IST_INDEX);
    }

    idt.simd_floating_point.set_handler_fn(isr::isr_simd_fp);
    idt.virtualization.set_handler_fn(isr::isr_virtualization);
}

fn configure_irqs(idt: &mut InterruptDescriptorTable) {
    // Timer uses the naked `timer_trampoline`. It captures the full
    // 15-GPR + iretq-frame into the current PCB's `saved_user_context`
    // when preempting a CPL=3 capsule, so the scheduler resume hook
    // can iretq back into user mode via `restore_user_context_iretq`.
    // The other IRQs still go through `extern "x86-interrupt"`
    // wrappers and do not yet capture user GPRs (1.F-β..ω).
    //
    // SAFETY: `timer_trampoline` is a `#[unsafe(naked)] extern "C"`
    // function pinned in kernel text. Its address is stable.
    unsafe {
        idt[vectors::VECTOR_TIMER as usize]
            .set_handler_addr(VirtAddr::new(isr::timer_trampoline as *const () as u64));
    }
    idt[vectors::VECTOR_KEYBOARD as usize].set_handler_fn(isr::irq_keyboard);
    idt[vectors::VECTOR_MOUSE as usize].set_handler_fn(isr::irq_mouse);
    install_broker_irq_entries(idt);
}

fn install_broker_irq_entries(idt: &mut InterruptDescriptorTable) {
    use crate::arch::interrupt::broker::{vector_of, BROKER_VEC_COUNT, ENTRIES};
    for slot in 0..BROKER_VEC_COUNT {
        if let Some(vector) = vector_of(slot) {
            idt[vector as usize].set_handler_fn(ENTRIES[slot]);
        }
    }
}

fn configure_syscall(idt: &mut InterruptDescriptorTable) {
    idt[vectors::VECTOR_SYSCALL as usize]
        .set_handler_fn(isr::irq_syscall)
        .set_privilege_level(PrivilegeLevel::Ring3);
}
