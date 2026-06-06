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

use super::correlation::take_for_reply;
use super::reply_inbox;

fn trace(caller_pid: u32, dest_pid: u64, label: &[u8], rc: i64, inbox: &str) {
    if caller_pid != 0x9 && dest_pid != 0x17 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-REPLY] from=");
    crate::sys::serial::trace_hex(caller_pid as u64);
    crate::sys::serial::trace(b" to=");
    crate::sys::serial::trace_hex(dest_pid);
    crate::sys::serial::trace(b" inbox=");
    crate::sys::serial::trace(inbox.as_bytes());
    crate::sys::serial::trace(b" ");
    crate::sys::serial::trace(label);
    crate::sys::serial::trace(b" rc=");
    if rc < 0 {
        crate::sys::serial::trace(b"-");
        crate::sys::serial::trace_dec((-rc) as u64);
    } else {
        crate::sys::serial::trace_dec(rc as u64);
    }
    crate::sys::serial::traceln(b"");
}

pub fn sys_ipc_reply(dest_pid: u64, buf: u64, len: usize) -> i64 {
    if len == 0 || dest_pid == 0 || dest_pid > u32::MAX as u64 {
        trace(current_pid().unwrap_or(0), dest_pid, b"inval", ERRNO_INVAL, "");
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_read(buf, len).is_err() {
        trace(current_pid().unwrap_or(0), dest_pid, b"validate", ERRNO_FAULT, "");
        return ERRNO_FAULT;
    }
    let mut data = alloc::vec![0u8; len];
    if crate::usercopy::copy_from_user(buf, &mut data).is_err() {
        trace(current_pid().unwrap_or(0), dest_pid, b"copy", ERRNO_FAULT, "");
        return ERRNO_FAULT;
    }
    let caller_pid = current_pid().unwrap_or(0);
    let dest = reply_inbox::for_pid(dest_pid as u32);
    let from = alloc::format!("proc.{}", caller_pid);
    let mut msg = match IpcMessage::new(&from, &dest, &data) {
        Ok(msg) => msg,
        Err(_) => {
            trace(caller_pid, dest_pid, b"message", ERRNO_NOMEM, &dest);
            return ERRNO_NOMEM;
        }
    };
    msg.correlation = take_for_reply(dest_pid as u32);
    let rc = match try_enqueue_strict(&dest, msg) {
        Ok(()) => {
            crate::sched::wake_process(dest_pid as u32);
            0
        }
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => ERRNO_NOENT,
        Err(StrictEnqueueError::QueueFull(_)) => ERRNO_BUSY,
    };
    let mut woke = false;
    if rc == 0 && crate::sched::is_sleeping(dest_pid as u32) {
        crate::sched::wake_process(dest_pid as u32);
        woke = true;
    }
    trace(caller_pid, dest_pid, if woke { b"enqueue wake=1" } else { b"enqueue wake=0" }, rc, &dest);
    rc
}
