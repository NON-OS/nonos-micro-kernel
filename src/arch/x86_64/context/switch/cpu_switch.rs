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

//! Standard `ret`-based kernel context switch. Saves callee-saved regs + the
//! return address on the outgoing kernel stack, stores rsp in `*prev_rsp_slot`,
//! loads `next_rsp`, pops, and `ret`s into the next task's resume point. This
//! replaces the setjmp/longjmp + `INTERRUPT_SAVED_CONTEXTS` mechanism. Not yet
//! routed (phase 2): exercised only by the self-test until the phase-4 cutover.

/// # Safety
/// `prev_rsp_slot` must be a valid `*mut u64`; `next_rsp` must point at a frame
/// previously produced by `cpu_switch` or `build_initial_switch_frame` on a live
/// kernel stack. Must run with interrupts disabled.
#[unsafe(naked)]
pub(crate) unsafe extern "C" fn cpu_switch(prev_rsp_slot: *mut u64, next_rsp: u64) {
    core::arch::naked_asm!(
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "mov [rdi], rsp",
        "mov rsp, rsi",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "ret",
    );
}

/// Seed a fresh kernel stack with a fake `cpu_switch` frame so the first
/// `cpu_switch` into the task `ret`s into `entry`. Layout (low→high):
/// r15,r14,r13,r12,rbx,rbp (zeroed), then `entry` as the ret target. The
/// target lands with `rsp` ≡ 8 (mod 16) — the SysV state right after a `call`
/// — so its SSE spills stay 16-aligned. Returns the value for `pcb.kernel_rsp`.
pub(crate) fn build_initial_switch_frame(stack_top: u64, entry: u64) -> u64 {
    let entry_rsp = (stack_top & !0xF) - 8;
    let rsp = entry_rsp - 7 * 8;
    unsafe {
        let p = rsp as *mut u64;
        for i in 0..6 {
            core::ptr::write(p.add(i), 0);
        }
        core::ptr::write(p.add(6), entry);
    }
    rsp
}
