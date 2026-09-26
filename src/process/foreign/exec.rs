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

//! `MkForeignExec`: the same guest, a different program.

use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};

use super::exec_context::fresh;
use super::peer_guard::{in_user_half, pid_arg};

type Saved = Option<crate::arch::context::SavedUser>;

/// What a parked guest receives when its supervisor has replaced the program
/// under it.
pub(super) const EXECED: u64 = u64::MAX;

pub fn sys_foreign_exec(pid: u64, entry: u64, rsp: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = match pid_arg(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if super::registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    if rsp == 0 || !in_user_half(entry, 1) || !in_user_half(rsp, 1) {
        return ERRNO_INVAL;
    }
    let Some(previous) = swap(pid, Some(fresh(entry, rsp))) else {
        return ERRNO_INVAL;
    };
    drop_tls(pid);
    // Answering is what releases the guest.
    match super::trap_reply::answer_raw(pid, EXECED) {
        0 => 0,
        err => {
            swap(pid, previous);
            err
        }
    }
}

/// Put a context in place and hand back the one it displaced.
fn swap(pid: u32, ctx: Saved) -> Option<Saved> {
    crate::process::with_process(pid, |p| {
        core::mem::replace(&mut *p.saved_user_context.lock(), ctx)
    })
}

/// Forget the thread pointer the replaced runtime set: the scheduler writes
/// the control block's base on every switch, so leaving it would put the new
/// image back on the old TLS the first time it is preempted.
fn drop_tls(pid: u32) {
    crate::process::with_process(pid, |pcb| pcb.set_tls_base(0));
}
