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

//! Parent-to-child stdin channel, the write-side counterpart to the
//! mirrored stdout drain in `proc_output.rs`. A launcher (the
//! terminal) feeds one message to a running child capsule's
//! `stdin.<pid>` inbox; the child drains it with `MkStdinRead`. Same
//! inbox-naming convention and parent gate as `sys_proc_output`.

use super::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};
use crate::process::{current_pid, get_parent_pid};

pub fn sys_proc_input(pid: u64, buf_ptr: u64, buf_len: usize) -> i64 {
    if buf_ptr == 0 || buf_len == 0 || pid == 0 || pid > u32::MAX as u64 {
        return ERRNO_INVAL;
    }
    if buf_len > crate::ipc::nonos_channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    // Only the parent that loaded the capsule may feed its stdin. Without
    // this any IPC-capable capsule could inject input into another
    // capsule's stdin by passing its pid, since the inbox name is derived
    // from the argument.
    let target = pid as u32;
    let caller = current_pid().unwrap_or(0);
    if caller == 0 || get_parent_pid(target) != Some(caller) {
        return ERRNO_PERM;
    }
    if crate::usercopy::validate_user_read(buf_ptr, buf_len).is_err() {
        return ERRNO_FAULT;
    }
    let mut data = alloc::vec![0u8; buf_len];
    if crate::usercopy::copy_from_user(buf_ptr, &mut data).is_err() {
        return ERRNO_FAULT;
    }
    let name = alloc::format!("stdin.{}", target);
    let from = alloc::format!("proc.{}", caller);
    let msg = match crate::ipc::nonos_channel::IpcMessage::new(&from, &name, &data) {
        Ok(m) => m,
        Err(_) => return ERRNO_INVAL,
    };
    match crate::ipc::nonos_inbox::try_enqueue_strict(&name, msg) {
        Ok(()) => data.len() as i64,
        Err(crate::ipc::nonos_inbox::StrictEnqueueError::MissingInbox)
        | Err(crate::ipc::nonos_inbox::StrictEnqueueError::DeadOwner) => ERRNO_NOENT,
        Err(crate::ipc::nonos_inbox::StrictEnqueueError::QueueFull(_)) => ERRNO_BUSY,
    }
}

pub fn sys_stdin_read(buf_ptr: u64, buf_len: usize) -> i64 {
    if buf_ptr == 0 || buf_len == 0 {
        return ERRNO_INVAL;
    }
    let caller = current_pid().unwrap_or(0);
    if caller == 0 {
        return ERRNO_PERM;
    }
    let name = alloc::format!("stdin.{}", caller);
    let Some(msg) = crate::ipc::nonos_inbox::try_dequeue_existing(&name) else {
        return 0;
    };
    let n = msg.data.len().min(buf_len);
    if crate::usercopy::copy_to_user(buf_ptr, &msg.data[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}
