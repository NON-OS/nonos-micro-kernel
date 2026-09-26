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

//! The supervisor's answer, and what happens to a guest left without one.

use super::peer_guard::pid_arg;
use super::registry;
use super::trap_table::PARKED;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

// A guest whose supervisor died is not left asleep forever and is not told its
// call succeeded.
pub(super) const ABANDONED: u64 = ERRNO_NOENT as u64;

/// `MkForeignReply`: answer one parked guest. Refused unless the caller is
/// that guest's recorded supervisor, so a pid alone buys nothing.
pub fn sys_foreign_reply(pid: u64, value: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = match pid_arg(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    answer_raw(pid, value)
}

/// Hand a parked guest its value and wake it. The permission check is
/// the caller's: `exec` has made it already, on the same terms.
pub(super) fn answer_raw(pid: u32, value: u64) -> i64 {
    let mut parked = PARKED.lock();
    let Some(entry) = parked.iter_mut().find(|p| p.frame.pid == pid && p.answer.is_none()) else {
        return ERRNO_NOENT;
    };
    entry.answer = Some(value);
    drop(parked);
    crate::sched::wake_process(pid);
    0
}

/// Drop every frame belonging to a process that is gone, so a reused
/// pid cannot collect an answer left behind by its predecessor.
pub(super) fn forget(pid: u32) {
    PARKED.lock().retain(|p| p.frame.pid != pid);
}

/// Release every frame belonging to a guest whose supervisor has gone.
pub(super) fn abandon(pid: u32) {
    let mut parked = PARKED.lock();
    for entry in parked.iter_mut().filter(|p| p.frame.pid == pid) {
        entry.answer = Some(ABANDONED);
    }
    drop(parked);
    crate::sched::wake_process(pid);
}
