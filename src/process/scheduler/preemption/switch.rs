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
use super::state::SCHEDULER_STATS;
use core::sync::atomic::Ordering;

// Timer-driven preemption. Runs synchronously inside the timer ISR on the
// interrupted task's kernel stack, interrupts off. `switch_to_process` parks
// this task via `cpu_switch`; it returns here only once the task is scheduled
// again, after which the ISR epilogue iretqs back to userspace through the
// frame still resident on this kernel stack.
pub(crate) fn preempt_current_process() {
    use crate::process::nonos_core::{current_pid, save_fpu_state, ProcessState, PROCESS_TABLE};

    let curr_pid = match current_pid() {
        Some(pid) => pid,
        None => return,
    };

    save_fpu_state(curr_pid);
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(curr_pid) {
        pcb.saved_user_stack.store(crate::smp::percpu::user_stack(), Ordering::Release);
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Running) {
            *state = ProcessState::Ready;
        }
    }
    add_to_run_queue(curr_pid);

    if let Some(next) = select_next_process() {
        if next != curr_pid {
            SCHEDULER_STATS.context_switches.fetch_add(1, Ordering::Relaxed);
            SCHEDULER_STATS.preemptions.fetch_add(1, Ordering::Relaxed);
            switch_to_process(next);
        }
    }

    if let Some(pcb) = PROCESS_TABLE.find_by_pid(curr_pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Ready) {
            *state = ProcessState::Running;
        }
    }
}
