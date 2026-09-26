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

//! A second thread inside a guest.

use super::peer_guard::{in_user_half, pid_arg};
use crate::process::core::{admit_thread, spawn_thread_parked};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

/// `MkForeignThread`: a thread in `pid`, sharing its address space and
/// supervised by the same caller.
pub fn sys_foreign_thread(pid: u64, entry: u64, rsp: u64, tls: u64) -> i64 {
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
    /*
     * Ring three addresses only, and the thread pointer among them: the switch
     * writes that one to an MSR that faults in ring zero on a non-canonical
     * value.
     */
    if !in_user_half(entry, 1) || !in_user_half(rsp, 0) {
        return ERRNO_INVAL;
    }
    if tls != 0 && !in_user_half(tls, 1) {
        return ERRNO_INVAL;
    }
    let Ok(tid) = spawn_thread_parked(pid, entry, rsp) else {
        return ERRNO_NOMEM;
    };
    if tls != 0 {
        crate::process::with_process(tid, |pcb| pcb.set_tls_base(tls));
    }
    if !super::registry::insert(tid, caller) {
        crate::process::exit::teardown(tid, ERRNO_NOMEM as i32, false);
        return ERRNO_NOMEM;
    }
    admit_thread(tid);
    tid as i64
}
