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

//! Naked #PF trampoline. `page_fault::handle` reads per-CPU data through
//! `gs:`-relative addressing; entered from CPL=3 via a plain
//! `extern "x86-interrupt"` wrapper it runs on the user GS base (0) and
//! faults on its own per-CPU access, looping forever. This stub `swapgs`-es
//! when the fault came from CPL=3 so the handler runs on the kernel
//! per-CPU base, mirroring `timer_trampoline`. #PF pushes an error code,
//! so the iretq frame sits one word above it: error_code[0], rip[8],
//! cs[16], rflags[24], rsp[32], ss[40].

use x86_64::structures::idt::InterruptStackFrame;

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn page_fault_trampoline() {
    core::arch::naked_asm!(
        "test byte ptr [rsp + 16], 3",
        "jz 1f",
        "swapgs",
        "1:",
        // User RFLAGS may carry DF=1; Rust handlers require forward string ops.
        "cld",
        "push rax",
        "push rcx",
        "push rdx",
        "push rbx",
        "push rbp",
        "push rsi",
        "push rdi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "mov rbp, rsp",
        "lea rdi, [rbp + 128]",
        "mov rsi, [rbp + 120]",
        "sub rsp, 528",
        "and rsp, -16",
        "fxsave [rsp]",
        "mov rbx, rsp",
        "call {handler}",
        "fxrstor [rbx]",
        "mov rsp, rbp",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdi",
        "pop rsi",
        "pop rbp",
        "pop rbx",
        "pop rdx",
        "pop rcx",
        "pop rax",
        "test byte ptr [rsp + 16], 3",
        "jz 2f",
        "swapgs",
        "2:",
        "add rsp, 8",
        "iretq",
        handler = sym page_fault_trap_handler,
    );
}

extern "C" fn page_fault_trap_handler(frame: *const InterruptStackFrame, error_code: u64) {
    let isf = unsafe { core::ptr::read(frame) };
    crate::interrupts::handlers::page_fault(isf, error_code);
}
