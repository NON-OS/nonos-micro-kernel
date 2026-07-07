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

use super::errnos::{ERRNO_CHILD, ERRNO_TIMEDOUT};
use crate::process::{current_pid, exit_status, get_parent_pid};

const SLICE_MS: u64 = 5;

pub fn sys_wait(pid: u64, timeout_ms: u64) -> i64 {
    let caller = current_pid().unwrap_or(0);
    if caller == 0 || get_parent_pid(pid as u32) != Some(caller) {
        return ERRNO_CHILD;
    }
    let deadline = crate::time::timestamp_millis().saturating_add(timeout_ms);
    loop {
        if let Some(code) = exit_status(pid as u32) {
            return code as i64;
        }
        if crate::time::timestamp_millis() >= deadline {
            return ERRNO_TIMEDOUT;
        }
        sleep_slice(caller);
    }
}

fn sleep_slice(pid: u32) {
    let wake = crate::time::timestamp_millis().saturating_add(SLICE_MS);
    crate::sched::sleep_until(pid, wake);
    crate::sched::yield_now();
}
