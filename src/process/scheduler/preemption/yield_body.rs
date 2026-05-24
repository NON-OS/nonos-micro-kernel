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
use super::state::CURRENT_TIME_SLICE;
use core::sync::atomic::Ordering;

/// Voluntary-yield body. Runs with interrupts already disabled by the
/// caller. The contract backend dispatches `SwitchIntent::Yield` here.
/// `switch_to_process` parks this task via `cpu_switch` and returns here
/// only once it is scheduled again, already marked Running by the resumer.
pub(crate) fn perform_yield_inline() {
    use crate::process::nonos_core::{current_pid, ProcessState, PROCESS_TABLE};

    let Some(pid) = current_pid() else { return };

    crate::process::nonos_core::save_fpu_state(pid);

    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        pcb.saved_user_stack
            .store(crate::smp::percpu::user_stack(), Ordering::Release);
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Running) {
            *state = ProcessState::Ready;
        }
    }

    add_to_run_queue(pid);
    CURRENT_TIME_SLICE.store(0, Ordering::Relaxed);

    if let Some(next) = select_next_process() {
        if next != pid {
            switch_to_process(next);
        }
    }

    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        let mut state = pcb.state.lock();
        if matches!(*state, ProcessState::Ready) {
            *state = ProcessState::Running;
        }
    }
}
