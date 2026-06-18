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

extern crate alloc;

use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox::{try_enqueue_strict, StrictEnqueueError};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{
    ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOMEM,
};

fn trace(caller_pid: u32, dest_pid: u64, len: usize) {
    if caller_pid != 0x17 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-RPLY] from=");
    crate::sys::serial::trace_hex(caller_pid as u64);
    crate::sys::serial::trace(b" to=");
    crate::sys::serial::trace_hex(dest_pid);
    crate::sys::serial::trace(b" len=");
    crate::sys::serial::trace_dec(len as u64);
    crate::sys::serial::traceln(b"");
}

// `MkIpcSendToPid` delivers `buf` to the destination pid's default
// per-process inbox `proc.<pid>`. Used by servers replying to a
// `MkIpcRecvFrom` caller without going through the named-endpoint
// registry. The kernel still records the sender in the message
// envelope so the receiver can chain a follow-up reply.
pub fn sys_ipc_send_to_pid(dest_pid: u64, buf: u64, len: usize) -> i64 {
    if len == 0 || len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    if dest_pid == 0 || dest_pid > u32::MAX as u64 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_read(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    let mut data = alloc::vec![0u8; len];
    if crate::usercopy::copy_from_user(buf, &mut data).is_err() {
        return ERRNO_FAULT;
    }
    let caller_pid = current_pid().unwrap_or(0);
    trace(caller_pid, dest_pid, len);
    let dest = alloc::format!("proc.{}", dest_pid as u32);
    let from = alloc::format!("proc.{}", caller_pid);
    let msg = IpcMessage::new(&from, &dest, &data).map_err(|_| ERRNO_NOMEM as i64);
    let msg = match msg {
        Ok(m) => m,
        Err(e) => return e,
    };
    let rc = match try_enqueue_strict(&dest, msg) {
        Ok(()) => 0,
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => ERRNO_NOENT,
        Err(StrictEnqueueError::QueueFull(_)) => ERRNO_BUSY,
    };
    if rc == 0 && crate::sched::is_sleeping(dest_pid as u32) {
        crate::sched::wake_process(dest_pid as u32);
    }
    rc
}
