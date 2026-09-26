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

//! Making a built guest runnable.

use super::peer_guard::{in_user_half, pid_arg};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};

// `rsp` of zero asks for the kernel's own user stack.
pub fn sys_foreign_start(pid: u64, entry: u64, rsp: u64) -> i64 {
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
     * An entry of zero means the guest already holds the state it should wake
     * in, which is what a fork leaves behind.
     */
    if entry == 0 && rsp == 0 {
        return super::resume::resume(pid);
    }
    /*
     * A guest runs in ring three, so a kernel entry or stack would fault on
     * its first instruction rather than escalate.
     */
    if !in_user_half(entry, 1) || (rsp != 0 && !in_user_half(rsp, 0)) {
        return ERRNO_INVAL;
    }
    super::start_context::install(pid, entry, rsp)
}
