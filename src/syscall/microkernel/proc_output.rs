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

//! Drain one stdout line a child capsule mirrored into its `proc.<pid>`
//! inbox (see the MkDebug handler). A launcher polls this to show a
//! loaded capsule's output in its own window. Returns the number of
//! bytes copied, 0 when the inbox is empty, or a negative errno.

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use crate::process::{current_pid, get_parent_pid};

pub fn sys_proc_output(pid: u64, buf_ptr: u64, buf_len: usize) -> i64 {
    if buf_ptr == 0 || buf_len == 0 || pid == 0 || pid > u32::MAX as u64 {
        return ERRNO_INVAL;
    }
    // Only the parent that loaded the capsule may drain its mirrored stdout.
    // Without this any IPC-capable capsule could read another capsule's output
    // by passing its pid, since the inbox name is derived from the argument.
    let target = pid as u32;
    let caller = current_pid().unwrap_or(0);
    if caller == 0 || parent_of(target) != Some(caller) {
        return ERRNO_PERM;
    }
    let name = alloc::format!("proc.{}", target);
    let Some(msg) = crate::ipc::nonos_inbox::try_dequeue_existing(&name) else {
        crate::process::exit::postmortem::release(target);
        return 0;
    };
    let n = msg.data.len().min(buf_len);
    if crate::usercopy::copy_to_user(buf_ptr, &msg.data[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}

/// The pid that launched `pid`, from the PCB while it lives and from the
/// post-mortem stdout retention afterwards, so a parent can still drain a
/// short-lived child that has already been finalized.
fn parent_of(pid: u32) -> Option<u32> {
    get_parent_pid(pid).or_else(|| crate::process::exit::postmortem::retained_parent(pid))
}
