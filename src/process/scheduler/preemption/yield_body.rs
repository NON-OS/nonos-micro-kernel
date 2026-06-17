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

use super::super::dispatch::add_to_run_queue;
use super::super::selection::{select_next_process, switch_to_process};
use super::save_syscall_user_rsp;
use super::state::CURRENT_TIME_SLICE;
use core::sync::atomic::Ordering;

/// Voluntary-yield body. Runs with interrupts already disabled by the
/// caller. The contract backend dispatches `SwitchIntent::Yield` here.
#[inline(never)]
pub(crate) fn perform_yield_inline() {
    use crate::process::nonos_core::{current_pid, ProcessState, PROCESS_TABLE};

    let Some(pid) = current_pid() else { return };

    let mut ctx: crate::sched::Context = unsafe { core::mem::zeroed() };
    crate::sched::Context::clear_restored_flag();
    unsafe { crate::sched::Context::save_to(&mut ctx as *mut crate::sched::Context) };
    if crate::sched::Context::was_just_restored() {
        return;
    }

    save_syscall_user_rsp(pid);
    crate::process::nonos_core::save_interrupt_context(pid, ctx);
    crate::process::nonos_core::save_fpu_state(pid);

    let runnable = if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Running) {
            *state = ProcessState::Ready;
        }
        matches!(*state, ProcessState::Ready)
    } else {
        false
    };

    if runnable {
        add_to_run_queue(pid);
    }
    CURRENT_TIME_SLICE.store(0, Ordering::Relaxed);

    loop {
        if let Some(next) = select_next_process() {
            if next != pid {
                switch_to_process(next);
            } else if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
                let mut state = pcb.state.lock();
                if matches!(*state, ProcessState::Ready) {
                    *state = ProcessState::Running;
                }
            }
            return;
        }
        idle_until_interrupt();
    }
}

// Nothing is runnable: every process is parked on a timeout or an
// IRQ/IPC wake. Yield used to plain-return here, which sent the
// caller's recv/wait loop spinning at CPL=0 with IF=0 (SFMASK) —
// the timer could never fire, so time froze and no sleeper could
// ever wake. The `sti; hlt` pair is the canonical race-free idle:
// STI's one-instruction shadow means a wake that is already
// pending still lands inside the HLT, and the handler (timer tick
// or broker IRQ) refills the run queue before control returns.
fn idle_until_interrupt() {
    unsafe {
        core::arch::asm!("sti", "hlt", "cli", options(nomem, nostack));
    }
}
