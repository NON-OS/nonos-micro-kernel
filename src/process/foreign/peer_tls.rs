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


//! Setting a guest thread's thread pointer.

use super::peer_guard::{in_user_half, pid_arg};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

/// `MkPeerTls`: the FS base `pid` wakes with from now on.
pub fn sys_peer_tls(pid: u64, base: u64) -> i64 {
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
     * The context switch writes this straight to MSR_FS_BASE, and that
     * instruction faults in ring zero for a non-canonical value.
     */
    if !in_user_half(base, 1) {
        return ERRNO_INVAL;
    }
    match crate::process::with_process(pid, |pcb| pcb.set_tls_base(base)) {
        Some(()) => 0,
        None => ERRNO_NOENT,
    }
}
