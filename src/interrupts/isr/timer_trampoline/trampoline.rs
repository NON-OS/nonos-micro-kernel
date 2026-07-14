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

use super::handler::timer_trap_handler;

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn timer_trampoline() {
    core::arch::naked_asm!(
        // Iretq frame is at: rip[0], cs[8], rflags[16], rsp[24], ss[32].
        // Inspect the saved CS for RPL=3 to decide swapgs.
        "test byte ptr [rsp + 8], 3",
        "jz 1f",
        "swapgs",
        "1:",
        // User RFLAGS may carry DF=1; Rust handlers require forward string ops.
        "cld",
        // Push 15 GPRs in the order that produces the UserContext
        // layout described in the module-level comment. rax first,
        // r15 last, so r15 ends up at the lowest address (offset 0).
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
        // After the 15 pushes, rsp ≡ 8 (mod 16); `call` adds another
        // 8 byte return address, giving rsp ≡ 0 (mod 16) at handler
        // entry — exactly what the SysV AMD64 ABI requires.
        "mov rbp, rsp",
        "mov rdi, rsp",
        // Save the interrupted SIMD state before any Rust runs. The
        // handler chain clobbers volatile XMM and would otherwise
        // corrupt a preempted kernel-side SSE computation (blake3).
        // `rbp`/`rbx` are callee-saved across the C call; their
        // original values are already on the stack for the pops.
        "sub rsp, 528",
        "and rsp, -16",
        "fxsave [rsp]",
        "mov rbx, rsp",
        "call {handler}",
        "fxrstor [rbx]",
        "mov rsp, rbp",
        // Pop GPRs in reverse, restoring user state.
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
        // After 15 pops, rsp is back to the iretq frame. Inspect the CS
        // again — it can have been overwritten by a different value
        // if a handler chose to redirect, but for the same-PCB return
        // case it is still the original CS we entered with.
        "test byte ptr [rsp + 8], 3",
        "jz 2f",
        "swapgs",
        "2:",
        "iretq",
        handler = sym timer_trap_handler,
    );
}
