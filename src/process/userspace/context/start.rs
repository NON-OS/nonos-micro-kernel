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

//! Entering a thread that has never run. Unlike `switch_context` there is no
//! outgoing context to save: whatever called this is not coming back.

use crate::process::nonos_context::CpuContext;

/// # Safety
/// `ctx` must be a live `CpuContext` holding the thread's entry point, and
/// `kernel_stack_top` the top of a stack that thread owns for its lifetime.
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn switch_to_new_thread(ctx: *const CpuContext, kernel_stack_top: u64) {
    core::arch::naked_asm!(
        // RDI = ctx, RSI = kernel_stack_top.
        "mov rsp, rsi",
        "mov r15, [rdi + 0]",
        "mov r14, [rdi + 8]",
        "mov r13, [rdi + 16]",
        "mov r12, [rdi + 24]",
        "mov rbx, [rdi + 32]",
        "mov rbp, [rdi + 40]",
        "mov rax, [rdi + 64]",
        "push rax",
        "popfq",
        "jmp [rdi + 48]",
    );
}
