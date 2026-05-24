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

use crate::process::core::{CURRENT_PID, PROCESS_TABLE};

use super::cpu_switch::cpu_switch;
use super::resume_env::prepare_resume;

// Unified resume: install the next task's environment, then swap kernel
// stacks via `cpu_switch`. A never-run task lands on its `first_entry_
// trampoline` frame (seeded in setup); an already-run task resumes exactly
// where it parked inside its own preempt/yield call, and the iretq/SYSRET
// frame still on its kernel stack carries it back to userspace. The outgoing
// task's resume point is saved into its own `kernel_rsp` by `cpu_switch`.
pub(crate) fn switch_to_user_pcb_x86_64(next_pid: u32) {
    let prev_pid = CURRENT_PID.load(Ordering::SeqCst);
    if next_pid == prev_pid {
        return;
    }
    let next = match PROCESS_TABLE.find_by_pid(next_pid) {
        Some(p) => p,
        None => return,
    };
    let next_rsp = next.kernel_rsp.load(Ordering::Acquire);
    if next_rsp == 0 {
        return;
    }

    let prev = PROCESS_TABLE.find_by_pid(prev_pid);
    if !prepare_resume(&next, next_pid) {
        return;
    }

    match prev {
        Some(p) => unsafe { cpu_switch(p.kernel_rsp.as_ptr(), next_rsp) },
        None => {
            let mut discard = 0u64;
            unsafe { cpu_switch(&mut discard as *mut u64, next_rsp) }
        }
    }
}
