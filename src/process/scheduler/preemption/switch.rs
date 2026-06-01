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
use super::state::SCHEDULER_STATS;
use core::sync::atomic::{AtomicU32, Ordering};

static PREEMPT_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 0x1b | 0x1c | 0x26 | 0x27)
        || PREEMPT_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40
    {
        return;
    }
    crate::sys::serial::print(b"[PREEMPT] ");
    crate::sys::serial::println(label);
}

#[inline(never)]
pub(crate) fn preempt_current_process() {
    use crate::process::nonos_core::{current_pid, save_fpu_state, ProcessState, PROCESS_TABLE};

    let curr_pid = match current_pid() {
        Some(pid) => pid,
        None => return,
    };
    trace(b"enter", curr_pid);

    save_fpu_state(curr_pid);
    crate::sched::Context::clear_restored_flag();
    let mut ctx: crate::sched::Context = unsafe { core::mem::zeroed() };
    unsafe { crate::sched::Context::save_to(&mut ctx as *mut crate::sched::Context) };
    if crate::sched::Context::was_just_restored() {
        trace(b"restored", curr_pid);
        return;
    }

    save_syscall_user_rsp(curr_pid);
    crate::process::nonos_core::save_interrupt_context(curr_pid, ctx);
    let runnable = if let Some(pcb) = PROCESS_TABLE.find_by_pid(curr_pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Running) {
            *state = ProcessState::Ready;
        }
        matches!(*state, ProcessState::Ready)
    } else {
        false
    };
    if runnable {
        add_to_run_queue(curr_pid);
    }

    match select_next_process() {
        Some(next) if next != curr_pid => {
            SCHEDULER_STATS.context_switches.fetch_add(1, Ordering::Relaxed);
            SCHEDULER_STATS.preemptions.fetch_add(1, Ordering::Relaxed);
            trace(b"switch away", curr_pid);
            switch_to_process(next);
        }
        _ => {
            if let Some(pcb) = PROCESS_TABLE.find_by_pid(curr_pid) {
                let mut state = pcb.state.lock();
                if matches!(*state, ProcessState::Ready) {
                    *state = ProcessState::Running;
                }
            }
        }
    }
}
