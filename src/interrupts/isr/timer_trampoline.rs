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
//! Naked timer-IRQ trampoline.
//!
//! Replaces the `extern "x86-interrupt" irq_timer` wrapper for the
//! timer vector. It saves the full general-purpose register set plus
//! the CPU-pushed iretq frame onto the kernel stack so the trap
//! handler can capture the user context of a CPL=3 capsule that was
//! preempted. Layout produced on the stack (low → high):
//!
//!     [rsp +   0] r15      ← last push
//!     [rsp +   8] r14
//!     [rsp +  16] r13
//!     [rsp +  24] r12
//!     [rsp +  32] r11
//!     [rsp +  40] r10
//!     [rsp +  48] r9
//!     [rsp +  56] r8
//!     [rsp +  64] rdi
//!     [rsp +  72] rsi
//!     [rsp +  80] rbp
//!     [rsp +  88] rbx
//!     [rsp +  96] rdx
//!     [rsp + 104] rcx
//!     [rsp + 112] rax      ← first push
//!     [rsp + 120] rip      ← CPU-pushed
//!     [rsp + 128] cs
//!     [rsp + 136] rflags
//!     [rsp + 144] rsp
//!     [rsp + 152] ss
//!
//! This is exactly the first 160 bytes of `process::userspace::types::
//! UserContext`. The trampoline hands a `*mut UserContext` to the
//! Rust C-ABI handler `timer_trap_handler` which decides whether to
//! capture the frame onto the current PCB and runs the existing
//! timer-tick body. On return from the handler the trampoline pops
//! the GPRs, `swapgs`-es back if returning to CPL=3, and `iretq`s.
//!
//! From CPL=0 the CPU does not switch to TSS.RSP0 — the trampoline
//! runs on whatever kernel stack was already current — and `swapgs`
//! is skipped on both entry and exit.

use crate::interrupts::apic;
use crate::interrupts::pic;
use crate::interrupts::safety::set_interrupt_context;
use crate::interrupts::stats;
use crate::interrupts::timer;
use crate::process::userspace::types::UserContext;

const TIMER_IRQ_LINE: u8 = 0;

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn timer_trampoline() {
    core::arch::naked_asm!(
        // Iretq frame is at: rip[0], cs[8], rflags[16], rsp[24], ss[32].
        // Test the saved CS for RPL=3 to decide swapgs.
        "test byte ptr [rsp + 8], 3",
        "jz 1f",
        "swapgs",
        "1:",
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
        "mov rdi, rsp",
        "call {handler}",
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
        // After 15 pops, rsp is back to the iretq frame. Test the CS
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

/// Rust C-ABI body of the timer trampoline.
///
/// On entry, `ctx` points at a stack-resident region whose layout
/// matches the first 160 bytes of `UserContext` (15 GPRs + iretq
/// frame). The pointer is valid only for the duration of this call;
/// the trampoline reuses the memory on return.
///
/// When the trap originated from CPL=3, this function snapshots the
/// frame onto the current PCB's `saved_user_context` so the scheduler
/// resume hook can iretq back into the capsule via
/// `restore_user_context_iretq`. Replaces any prior snapshot — a
/// later context write overwrites earlier ones, and the scheduler
/// `take()`s the most recent one.
#[no_mangle]
pub extern "C" fn timer_trap_handler(ctx: *mut UserContext) {
    // SAFETY: eK@nonos.systems — `ctx` was produced by the trampoline
    // above and points at 160 bytes of valid stack memory laid out as
    // the leading fields of `UserContext`. We read those fields here;
    // `fs_base` and `gs_base` (the trailing 16 bytes of the full
    // struct) are not read because the trampoline does not write them.
    let frame = unsafe { &*ctx };
    let from_user = (frame.cs & 3) == 3;

    if from_user {
        if let Some(pcb) = crate::process::current_process() {
            let snapshot = UserContext {
                r15: frame.r15,
                r14: frame.r14,
                r13: frame.r13,
                r12: frame.r12,
                r11: frame.r11,
                r10: frame.r10,
                r9: frame.r9,
                r8: frame.r8,
                rdi: frame.rdi,
                rsi: frame.rsi,
                rbp: frame.rbp,
                rbx: frame.rbx,
                rdx: frame.rdx,
                rcx: frame.rcx,
                rax: frame.rax,
                rip: frame.rip,
                cs: frame.cs,
                rflags: frame.rflags,
                rsp: frame.rsp,
                ss: frame.ss,
                fs_base: 0,
                gs_base: 0,
            };
            *pcb.saved_user_context.lock() = Some(snapshot);
        }
    }

    let _ctx_guard = set_interrupt_context();
    stats::increment_timer();
    timer::on_timer_interrupt();
    crate::kernel_core::process_spawn::drain_pending_kernel_stacks();
    send_eoi();
}

fn send_eoi() {
    if apic::is_enabled() {
        apic::send_eoi();
    } else {
        pic::send_eoi(TIMER_IRQ_LINE);
    }
}
