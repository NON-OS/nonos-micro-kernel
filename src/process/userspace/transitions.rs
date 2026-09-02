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

pub use super::asm::{
    jump_to_usermode, restore_user_context_iretq, return_to_usermode, sysret_to_usermode,
};
use super::types::ExecContext;

// CR4 belongs to `memory::mmu`, which brings SMEP, SMAP and UMIP up once at
// boot and reports what the hardware confirmed. This file used to carry its
// own copy of that write; it is gone rather than delegated, because a second
// place that can turn a protection on is a second place that can be reached
// with a different idea of what is already on.

pub fn exec_process(ctx: &ExecContext) -> ! {
    x86_64::instructions::interrupts::disable();
    unsafe {
        core::arch::asm!("mov cr3, {}", in(reg) ctx.cr3, options(nostack));
    }
    crate::security::speculation::kernel_exit();
    unsafe {
        jump_to_usermode(ctx.entry, ctx.stack, ctx.argc);
    }
}
