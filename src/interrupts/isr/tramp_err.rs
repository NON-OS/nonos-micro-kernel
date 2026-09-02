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

//! Trampoline for a vector the CPU pushes an error code for, so CS sits at
//! `[rsp+16]` and the code slot is discarded before `iretq`.

macro_rules! exc_tramp_err {
    ($tramp:ident, $shim:ident, $handler:path) => {
        #[unsafe(naked)]
        #[no_mangle]
        pub unsafe extern "C" fn $tramp() {
            core::arch::naked_asm!(
                "test byte ptr [rsp + 16], 3", "jz 1f", "swapgs", "1:",
                // User RFLAGS may carry DF=1; Rust handlers require forward string ops.
                "cld",
                "push rax\npush rcx\npush rdx\npush rbx\npush rbp\npush rsi\npush rdi\npush r8\npush r9\npush r10\npush r11\npush r12\npush r13\npush r14\npush r15\nmov rbp, rsp",
                "lea rdi, [rbp + 128]", "mov rsi, [rbp + 120]",
                "sub rsp, 528", "and rsp, -16", "fxsave [rsp]", "mov rbx, rsp",
                "call {h}",
                "fxrstor [rbx]",
                "mov rsp, rbp\npop r15\npop r14\npop r13\npop r12\npop r11\npop r10\npop r9\npop r8\npop rdi\npop rsi\npop rbp\npop rbx\npop rdx\npop rcx\npop rax",
                "test byte ptr [rsp + 16], 3", "jz 2f", "swapgs", "2:",
                "add rsp, 8", "iretq",
                h = sym $shim,
            );
        }
        extern "C" fn $shim(
            frame: *const x86_64::structures::idt::InterruptStackFrame,
            error_code: u64,
        ) {
            $handler(unsafe { core::ptr::read(frame) }, error_code);
        }
    };
}

pub(super) use exc_tramp_err;
