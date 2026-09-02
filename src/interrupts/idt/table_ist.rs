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

//! The exceptions that need a stack of their own.
//!
//! Each of these can be taken at a moment when the current stack cannot be
//! trusted, so the CPU is told to switch to a dedicated one: #DB in the
//! kernel-entry window after swapgs and before the stack switch, NMI nested on
//! another NMI, #DF on a stack overflow, #GP on a torn TSS.RSP0 mid context
//! switch, #PF on a guard page, #MC on hardware already misbehaving.
//!
//! `set_stack_index` is 0-based and adds one internally, while the `gdt`
//! constants are the 1-based hardware slots, hence the subtraction.
//!
//! NMI is the one entry here still on a plain wrapper. It is not reachable
//! from CPL=3 by a capsule, and deciding its swapgs from the saved CS would be
//! wrong anyway, since an NMI can land in the window where CS reads kernel and
//! GS is still the user base. Doing that correctly means reading GS_BASE in
//! the stub, which is worth doing on its own and not as a side effect here.

use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::VirtAddr;

use crate::arch::x86_64::gdt;
use crate::interrupts::isr;

/// # Safety
/// Every address below is a naked or `x86-interrupt` function pinned in kernel
/// text, and every IST index names a stack the per-CPU TSS already carries.
pub(super) unsafe fn configure(idt: &mut InterruptDescriptorTable) {
    unsafe {
        idt.debug
            .set_handler_addr(addr(isr::db_trampoline as *const ()))
            .set_stack_index(gdt::DEBUG_IST_INDEX - 1);
        idt.non_maskable_interrupt
            .set_handler_fn(isr::isr_nmi)
            .set_stack_index(gdt::NMI_IST_INDEX - 1);
        idt.double_fault
            .set_handler_addr(addr(isr::isr_double_fault as *const ()))
            .set_stack_index(gdt::DF_IST_INDEX - 1);
        idt.general_protection_fault
            .set_handler_addr(addr(isr::gpf_trampoline as *const ()))
            .set_stack_index(gdt::GP_IST_INDEX - 1);
        idt.page_fault
            .set_handler_addr(addr(isr::page_fault_trampoline as *const ()))
            .set_stack_index(gdt::PF_IST_INDEX - 1);
        idt.machine_check
            .set_handler_addr(addr(isr::isr_machine_check as *const ()))
            .set_stack_index(gdt::MC_IST_INDEX - 1);
    }
}

#[inline]
fn addr(f: *const ()) -> VirtAddr {
    VirtAddr::new(f as u64)
}
