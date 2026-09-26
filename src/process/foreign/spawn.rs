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

//! Creating a guest: a process, a kernel stack, and nothing else.

use alloc::format;

use crate::kernel_core::process_spawn::allocate_kernel_stack;
use crate::process::core::types::Priority;
use crate::process::core::{create_process_with_parent, ProcessState};
use crate::syscall::microkernel::errnos::{
    ERRNO_EXIST, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM,
};
use crate::usercopy::read_user_bytes;

const MAX_NAME: usize = 24;

/// `MkForeignSpawn`: an empty, capability-free process supervised by the
/// caller.
pub fn sys_foreign_spawn(name_ptr: u64, name_len: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    if name_len == 0 || name_len as usize > MAX_NAME {
        return ERRNO_INVAL;
    }
    let Ok(bytes) = read_user_bytes(name_ptr, name_len as usize) else {
        return ERRNO_FAULT;
    };
    let Ok(name) = core::str::from_utf8(&bytes) else {
        return ERRNO_INVAL;
    };
    match empty_guest(caller, name.as_bytes()) {
        Ok(pid) => pid as i64,
        Err(e) => e,
    }
}

/// The one place a guest comes into being.
pub(super) fn empty_guest(supervisor: u32, name: &[u8]) -> Result<u32, i64> {
    let tag = format!("foreign:{}", core::str::from_utf8(name).unwrap_or("guest"));
    let pid = create_process_with_parent(&tag, ProcessState::New, Priority::Normal, 0, None)
        .map_err(|_| ERRNO_NOMEM)?;
    if allocate_kernel_stack(pid).is_err() {
        return Err(ERRNO_NOMEM);
    }
    /*
     * Every process is born with its parent's capabilities bounded by the
     * ambient set, which for a guest of this capsule means core exec, IPC and
     * memory.
     */
    if crate::process::caps::install_spawn(pid, 0).is_none() {
        return Err(ERRNO_PERM);
    }
    if !super::registry::insert(pid, supervisor) {
        return Err(ERRNO_EXIST);
    }
    Ok(pid)
}
