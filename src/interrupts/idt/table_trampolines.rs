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

//! The exceptions a capsule can trigger, on the kernel's own stack.
//!
//! Each one is entered from CPL=3, where GS still holds the user base, so all
//! of them go through a naked trampoline that swapgs-es first. Without it the
//! handler, or anything it calls, reads gs-relative state from the user base:
//! the terminate path writes `gs:0x28` in the scheduler resume and faults, and
//! on #PF that faults again and storms.
//!
//! The error-code vectors are the ones the CPU pushes a code for; their
//! trampolines discard the slot before `iretq`. Which is which is fixed by the
//! architecture, not by us.

use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::VirtAddr;

use crate::interrupts::isr;

/// # Safety
/// Every address below is a `#[unsafe(naked)] extern "C"` function pinned in
/// kernel text, so its address is stable for the life of the kernel.
pub(super) unsafe fn configure(idt: &mut InterruptDescriptorTable) {
    unsafe {
        idt.divide_error.set_handler_addr(addr(isr::de_trampoline as *const ()));
        idt.breakpoint.set_handler_addr(addr(isr::bp_trampoline as *const ()));
        idt.overflow.set_handler_addr(addr(isr::of_trampoline as *const ()));
        idt.bound_range_exceeded.set_handler_addr(addr(isr::br_trampoline as *const ()));
        idt.invalid_opcode.set_handler_addr(addr(isr::ud_trampoline as *const ()));
        idt.device_not_available.set_handler_addr(addr(isr::nm_trampoline as *const ()));
        idt.invalid_tss.set_handler_addr(addr(isr::ts_trampoline as *const ()));
        idt.segment_not_present.set_handler_addr(addr(isr::np_trampoline as *const ()));
        idt.stack_segment_fault.set_handler_addr(addr(isr::ss_trampoline as *const ()));
        idt.x87_floating_point.set_handler_addr(addr(isr::mf_trampoline as *const ()));
        idt.alignment_check.set_handler_addr(addr(isr::ac_trampoline as *const ()));
        idt.simd_floating_point.set_handler_addr(addr(isr::xf_trampoline as *const ()));
        idt.virtualization.set_handler_addr(addr(isr::ve_trampoline as *const ()));
    }
}

#[inline]
fn addr(f: *const ()) -> VirtAddr {
    VirtAddr::new(f as u64)
}
