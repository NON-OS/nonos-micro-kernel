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

use super::definition::Context;

impl Context {
    #[unsafe(naked)]
    pub unsafe extern "C" fn save_to(ctx: *mut Context) {
        core::arch::naked_asm!(
            "mov [rdi], rax",
            "mov [rdi + 8], rbx",
            "mov [rdi + 16], rcx",
            "mov [rdi + 24], rdx",
            "mov [rdi + 32], rsi",
            "mov [rdi + 40], rdi",
            "mov [rdi + 48], rbp",
            "lea rax, [rsp + 8]",
            "mov [rdi + 56], rax",
            "mov [rdi + 64], r8",
            "mov [rdi + 72], r9",
            "mov [rdi + 80], r10",
            "mov [rdi + 88], r11",
            "mov [rdi + 96], r12",
            "mov [rdi + 104], r13",
            "mov [rdi + 112], r14",
            "mov [rdi + 120], r15",
            "mov rax, [rsp]",
            "mov [rdi + 128], rax",
            "pushfq",
            "pop rax",
            "mov [rdi + 136], rax",
            "ret",
        );
    }

    #[inline(never)]
    pub fn save() -> Self {
        let mut ctx: Context = unsafe { core::mem::zeroed() };
        unsafe { Self::save_to(&mut ctx as *mut Context) };
        ctx
    }
}
