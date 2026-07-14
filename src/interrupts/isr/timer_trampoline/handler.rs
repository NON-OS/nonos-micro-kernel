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

use super::send_eoi::send_eoi;
use crate::interrupts::safety::set_interrupt_context;
use crate::interrupts::stats;
use crate::interrupts::timer;
use crate::process::userspace::types::UserContext;

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
pub(crate) extern "C" fn timer_trap_handler(ctx: *mut UserContext) {
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
    // EOI before the tick work: on_timer_interrupt can preempt-switch away
    // inside this interrupt, and a deferred EOI leaves the timer vector
    // in-service at the LAPIC — no further ticks, sleep wakeups, or
    // preemption until the preempted context happens to resume. Sending it
    // here is safe: IF stays clear until the iretq, so the handler cannot
    // re-enter; a next tick pends and fires after the return.
    send_eoi();
    timer::on_timer_interrupt();
    crate::process::exit::drain_pending_teardowns();
    crate::kernel_core::process_spawn::drain_pending_kernel_stacks();
}
