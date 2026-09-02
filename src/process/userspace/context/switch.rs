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

//! Suspending one kernel thread and resuming another. The numbers below are
//! `CpuContext` field offsets; that layout and this file change together.

use crate::process::nonos_context::CpuContext;

/// # Safety
/// Both pointers must be live, aligned `CpuContext` values. `next_ctx` must
/// describe a suspended thread whose stack and instruction pointer are valid,
/// because control leaves through it and does not return here.
#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn switch_context(current_ctx: *mut CpuContext, next_ctx: *const CpuContext) {
    core::arch::naked_asm!(
        // RDI = current_ctx, RSI = next_ctx. Save the callee-saved set.
        "mov [rdi + 0], r15",
        "mov [rdi + 8], r14",
        "mov [rdi + 16], r13",
        "mov [rdi + 24], r12",
        "mov [rdi + 32], rbx",
        "mov [rdi + 40], rbp",
        // Resume point: the return address this call was reached through.
        "mov rax, [rsp]",
        "mov [rdi + 48], rax",
        // Stack pointer as it will be once that return address is consumed.
        "lea rax, [rsp + 8]",
        "mov [rdi + 56], rax",
        "pushfq",
        "pop rax",
        "mov [rdi + 64], rax",
        // Load the next thread's callee-saved set, flags and stack, then go.
        "mov r15, [rsi + 0]",
        "mov r14, [rsi + 8]",
        "mov r13, [rsi + 16]",
        "mov r12, [rsi + 24]",
        "mov rbx, [rsi + 32]",
        "mov rbp, [rsi + 40]",
        "mov rax, [rsi + 64]",
        "push rax",
        "popfq",
        "mov rsp, [rsi + 56]",
        "jmp [rsi + 48]",
    );
}
