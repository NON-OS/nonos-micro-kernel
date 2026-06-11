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

use core::sync::atomic::Ordering;

use crate::process::core::{clear_current_if, Pid, ProcessState, CURRENT_PID, PROCESS_TABLE};

pub fn teardown(pid: Pid, exit_code: i32, _by_signal: bool) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };
    if matches!(
        *pcb.state.lock(),
        ProcessState::Zombie(_) | ProcessState::Terminated(_)
    ) {
        return;
    }

    crate::kernel_core::surface_registry::release_owned_by_pid(pid);
    crate::kernel_core::surface_registry::attach_map::forget_pid(pid);
    let current = CURRENT_PID.load(Ordering::Acquire) == pid;
    crate::hardware::broker::release_all_for_pid(pid, current);
    crate::hardware::broker::irq_release_all_for_pid(pid);
    crate::hardware::broker::dma_release_all_for_pid(pid, current);
    crate::hardware::broker::pio_release_all_for_pid(pid);

    crate::kernel_core::process_spawn::defer_kernel_stack_release(pid);

    pcb.exit_code.store(exit_code, Ordering::Release);
    *pcb.state.lock() = ProcessState::Zombie(exit_code);
    crate::sched::remove_from_run_queue(pid);
    clear_current_if(pid);
    crate::process::scheduler::preemption::proc_ticks::clear(pid);
    super::pending::enqueue(pid);
}
