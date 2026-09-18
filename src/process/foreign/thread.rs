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

use crate::process::core::spawn_thread_in;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

/// `MkForeignThread`: a thread in `pid`, sharing its address space and
/// supervised by the same caller. `tls` is stored rather than applied:
/// the context switch loads FS base from the control block, so a thread
/// that has never run still wakes with it set, which it must, because a
/// C runtime touches thread-local storage before its first instruction
/// of program code.
pub fn sys_foreign_thread(pid: u64, entry: u64, rsp: u64, tls: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = pid as u32;
    if super::registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    if entry == 0 || rsp == 0 {
        return ERRNO_INVAL;
    }
    let Ok(tid) = spawn_thread_in(pid, entry, rsp) else {
        return ERRNO_NOMEM;
    };
    if tls != 0 {
        crate::process::with_process(tid, |pcb| pcb.set_tls_base(tls));
    }
    // Its unknown syscalls have to reach the same supervisor, or the
    // thread traps into ENOSYS while its siblings are being served.
    if !super::registry::insert(tid, caller) {
        return ERRNO_NOMEM;
    }
    tid as i64
}
