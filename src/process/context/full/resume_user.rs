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

// Ring-3 selectors with RPL=3. These must match the GDT layout in
// `arch::x86_64::gdt::constants` (SEL_USER_CODE = 0x20|3 = 0x23,
// SEL_USER_DATA = 0x18|3 = 0x1B). `iretq` loads CS from the code
// descriptor and SS from the data descriptor; swapping them makes the
// CPU fault with #GP on the return, so the values are pinned here.
const USER_CS: u64 = 0x23;
const USER_SS: u64 = 0x1B;

impl Context {
    /// Restore this context as the user-mode register state and `iretq`
    /// into it. All eighteen `Context` fields are loaded from `self`.
    pub fn resume_user(&self) -> ! {
        // SAFETY: ek@nonos.systems — `resume_user_asm` reads the
        // `Context` once, builds a five-word iret frame on the kernel
        // stack (SS=0x1B, RSP, RFLAGS|0x202, CS=0x23, RIP), loads every
        // GPR from the context, swapgs to restore user GS, then iretq.
        // The reserved RFLAGS bits 1 and 9 are forced on so the user
        // resumes with interrupts enabled and a legal RFLAGS.
        unsafe { resume_user_asm(self as *const Context) }
    }
}

#[unsafe(naked)]
unsafe extern "C" fn resume_user_asm(_ctx: *const Context) -> ! {
    core::arch::naked_asm!(
        "push {ss}",
        "push qword ptr [rdi + 56]",
        "push qword ptr [rdi + 136]",
        "or qword ptr [rsp], 0x202",
        "push {cs}",
        "push qword ptr [rdi + 128]",
        "mov rax, [rdi + 0]",
        "mov rbx, [rdi + 8]",
        "mov rcx, [rdi + 16]",
        "mov rdx, [rdi + 24]",
        "mov rsi, [rdi + 32]",
        "mov rbp, [rdi + 48]",
        "mov r8,  [rdi + 64]",
        "mov r9,  [rdi + 72]",
        "mov r10, [rdi + 80]",
        "mov r11, [rdi + 88]",
        "mov r12, [rdi + 96]",
        "mov r13, [rdi + 104]",
        "mov r14, [rdi + 112]",
        "mov r15, [rdi + 120]",
        "mov rdi, [rdi + 40]",
        "swapgs",
        "iretq",
        ss = const USER_SS,
        cs = const USER_CS,
    );
}
