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

//! Device interrupts and the software syscall vector.
//!
//! A device raises its line whenever it decides to, which is about as often
//! while a capsule is running as while the kernel is, so these are entered
//! from CPL=3 no less than the exceptions are and their stubs swapgs the same
//! way. The timer trampoline does more: it captures the full 15-GPR and iretq
//! frame into the current PCB's `saved_user_context`, so the scheduler resume
//! hook can iretq back into user mode through `restore_user_context_iretq`.
//!
//! The broker entries are `extern "x86-interrupt"` wrappers generated per
//! slot; they are driver-owned lines and do not capture user GPRs.

use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::{PrivilegeLevel, VirtAddr};

use super::vectors;
use crate::interrupts::isr;

/// # Safety
/// Every address below is a `#[unsafe(naked)] extern "C"` function pinned in
/// kernel text, so its address is stable for the life of the kernel.
pub(super) unsafe fn configure(idt: &mut InterruptDescriptorTable) {
    unsafe {
        idt[vectors::VECTOR_TIMER as usize]
            .set_handler_addr(addr(isr::timer_trampoline as *const ()));
        idt[vectors::VECTOR_KEYBOARD as usize]
            .set_handler_addr(addr(isr::keyboard_trampoline as *const ()));
        idt[vectors::VECTOR_MOUSE as usize]
            .set_handler_addr(addr(isr::mouse_trampoline as *const ()));
        idt[vectors::VECTOR_SYSCALL as usize]
            .set_handler_addr(addr(isr::int80_trampoline as *const ()))
            .set_privilege_level(PrivilegeLevel::Ring3);
    }
    install_broker_entries(idt);
}

fn install_broker_entries(idt: &mut InterruptDescriptorTable) {
    use crate::arch::interrupt::broker::{vector_of, BROKER_VEC_COUNT, ENTRIES};
    for slot in 0..BROKER_VEC_COUNT {
        if let Some(vector) = vector_of(slot) {
            idt[vector as usize].set_handler_fn(ENTRIES[slot]);
        }
    }
}

#[inline]
fn addr(f: *const ()) -> VirtAddr {
    VirtAddr::new(f as u64)
}
