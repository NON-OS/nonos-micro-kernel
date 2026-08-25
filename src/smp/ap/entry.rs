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

use super::idle::ap_idle_loop;
use crate::smp::state::{AP_STARTUP_BARRIER, CPU_DESCRIPTORS};
use crate::smp::types::CpuState;
use core::sync::atomic::Ordering;

/// Entered from the trampoline once this AP is in long mode on its own stack.
///
/// # Safety
/// Called exactly once per AP, by the trampoline, with `cpu_id` the index the
/// BSP wrote into that AP's boot context and a descriptor already published.
#[no_mangle]
pub unsafe extern "C" fn ap_entry(cpu_id: u32) {
    // This AP's LAPIC comes out of INIT/SIPI software-disabled. The mode and
    // MMIO mapping were adopted from the BSP before the SIPI, so only the
    // per-CPU register programming runs here.
    // SAFETY: eK@nonos.systems - runs once on this AP before any interrupt is
    // enabled, and touches only this CPU's own LAPIC registers.
    unsafe { crate::arch::x86_64::interrupt::apic::init_ap_lapic() };
    let apic_id = crate::arch::interrupt_controller::local_id();

    // Before anything else that could ask which CPU it is running on. The
    // answer comes out of this block through the per-CPU register, and that
    // register is installed here; until it is, this AP would be answered with
    // the boot CPU's number and would read and write the boot CPU's state.
    // `cpu_id` is the index the BSP wrote into this AP's boot context, so it
    // is known without having to look it up.
    crate::smp::percpu::init_ap(cpu_id as usize);

    // SAFETY: eK@nonos.systems - GDT and TSS must be loaded before anything
    // that can fault. `cpu_id` indexes this AP's own per-CPU structures, which
    // the BSP allocated before releasing it.
    unsafe {
        let _ = crate::arch::x86_64::cpu::init_ap(cpu_id as u16, apic_id);
    }

    // Its own block: the slot was handed to this CPU and is never reused.
    let _ = crate::arch::x86_64::gdt::arm_ap_guards(cpu_id);

    // The IDT the BSP runs on, built by `interrupts::init_idt`. This pointed
    // at the second IDT under `arch::x86_64::idt`, whose `init` has no caller,
    // so an AP loaded a table nothing filled in and triple-faulted on its
    // first timer tick. `load` only writes IDTR against the BSP.s table.
    crate::interrupts::idt::load_idt();

    // The BSP registered the IRQ-0 handler; each AP arms its own LAPIC timer.
    crate::arch::x86_64::interrupt::apic::preemption::install_on_ap();

    CPU_DESCRIPTORS[cpu_id as usize].set_state(CpuState::Online);
    AP_STARTUP_BARRIER.fetch_add(1, Ordering::Release);

    // SAFETY: eK@nonos.systems - the IDT, GDT, TSS and per-CPU state above are
    // all in place, so this CPU can now take an interrupt.
    unsafe {
        core::arch::asm!("sti", options(nostack, nomem));
    }

    ap_idle_loop(cpu_id);
}
