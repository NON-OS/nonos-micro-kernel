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

use alloc::collections::BTreeMap;
use core::sync::atomic::Ordering;
use spin::Mutex;

use crate::process::core::{clear_current_if, Pid, ProcessState, CURRENT_PID, PROCESS_TABLE};

// `finalize_teardown` removes the PCB from PROCESS_TABLE entirely, so
// exit_code is unreadable once a zombie is reaped; this log keeps it
// available to a parent calling sys_wait after that point. Entries are
// consumed (removed) on read, like POSIX waitpid, and the map is capped
// so fire-and-forget children that are never waited on can't grow it
// without bound.
static REAP_LOG: Mutex<BTreeMap<Pid, (Pid, i32)>> = Mutex::new(BTreeMap::new());
const REAP_LOG_CAP: usize = 64;

pub(crate) fn reap_exit_status(pid: Pid) -> Option<i32> {
    REAP_LOG.lock().remove(&pid).map(|(_, code)| code)
}

pub(crate) fn reap_exit_status_for(pid: Pid, parent: Pid) -> Option<i32> {
    let mut log = REAP_LOG.lock();
    match log.get(&pid) {
        Some(&(logged_parent, code)) if logged_parent == parent => {
            log.remove(&pid);
            Some(code)
        }
        _ => None,
    }
}

pub fn teardown(pid: Pid, exit_code: i32, _by_signal: bool) {
    let pcb = match PROCESS_TABLE.find_by_pid(pid) {
        Some(p) => p,
        None => return,
    };
    if matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_)) {
        return;
    }

    crate::kernel_core::surface_registry::release_owned_by_pid(pid);
    crate::kernel_core::surface_registry::attach_map::forget_pid(pid);
    let current = CURRENT_PID.load(Ordering::Acquire) == pid;
    crate::hardware::broker::release_all_for_pid(pid, current);
    crate::hardware::broker::irq_release_all_for_pid(pid);
    crate::hardware::broker::dma_release_all_for_pid(pid, current);
    crate::hardware::broker::pio_release_all_for_pid(pid);
    crate::syscall::microkernel::ipc::release_pending_replies_for_pid(pid);

    crate::kernel_core::process_spawn::defer_kernel_stack_release(pid);

    pcb.exit_code.store(exit_code, Ordering::Release);
    *pcb.state.lock() = ProcessState::Zombie(exit_code);
    {
        let mut reap_log = REAP_LOG.lock();
        if reap_log.len() >= REAP_LOG_CAP {
            if let Some((&oldest, _)) = reap_log.iter().next() {
                reap_log.remove(&oldest);
            }
        }
        reap_log.insert(pid, (pcb.parent_pid(), exit_code));
    }
    super::postmortem::retain(pid, pcb.parent_pid());
    crate::sched::remove_from_run_queue(pid);
    clear_current_if(pid);
    crate::process::scheduler::preemption::proc_ticks::clear(pid);
    super::pending::enqueue(pid);
}
