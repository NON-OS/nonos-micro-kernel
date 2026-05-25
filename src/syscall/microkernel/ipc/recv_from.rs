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

use crate::ipc::nonos_inbox;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_TIMEDOUT};
use core::sync::atomic::{AtomicU32, Ordering};

use super::inbox_name::resolve_for_recv;
use super::sender_pid::from_envelope;

static RECV_FROM_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 7 | 8 | 0x17 | 0x1b | 0x1c | 0x27)
        || RECV_FROM_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40
    {
        return;
    }
    crate::sys::serial::print(b"[IPC-RF] ");
    crate::sys::serial::println(label);
}

// `MkIpcRecvFrom`. Same drain semantics as `MkIpcRecv`, with an
// extra `sender_pid_out` user pointer written with the caller pid
// of the dequeued message (0 for kernel-internal senders) so the
// receiver can reply via `MkIpcSendToPid`.
pub fn sys_ipc_recv_from(
    endpoint: u64,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    sender_pid_out: u64,
) -> i64 {
    if len == 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    if sender_pid_out != 0
        && crate::usercopy::validate_user_write(sender_pid_out, core::mem::size_of::<u32>())
            .is_err()
    {
        return ERRNO_FAULT;
    }
    let pid = current_pid().unwrap_or(0);
    trace(b"enter", pid);
    let inbox_name = match resolve_for_recv(endpoint, pid) {
        Ok(name) => name,
        Err(e) => return e,
    };
    if !nonos_inbox::exists(&inbox_name) {
        trace(b"missing inbox", pid);
        return ERRNO_NOENT;
    }
    drain(buf, len, timeout_ms, sender_pid_out, &inbox_name)
}

fn drain(buf: u64, len: usize, timeout_ms: u64, sender_pid_out: u64, inbox: &str) -> i64 {
    let start = crate::time::timestamp_millis();
    let pid = current_pid().unwrap_or(0);
    loop {
        if let Some(msg) = nonos_inbox::try_dequeue_existing(inbox) {
            trace(b"dequeue", pid);
            let copy_len = msg.data.len().min(len);
            if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
                return ERRNO_FAULT;
            }
            if sender_pid_out != 0 {
                let bytes = from_envelope(&msg.from).to_le_bytes();
                if crate::usercopy::copy_to_user(sender_pid_out, &bytes).is_err() {
                    return ERRNO_FAULT;
                }
            }
            return copy_len as i64;
        }
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        if timeout_ms > 0 && elapsed >= timeout_ms {
            return ERRNO_TIMEDOUT;
        }
        trace(b"before yield", pid);
        crate::sched::yield_now();
        trace(b"after yield", pid);
    }
}
