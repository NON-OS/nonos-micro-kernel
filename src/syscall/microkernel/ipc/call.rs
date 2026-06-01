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

use crate::process::current_pid;

use super::recv::recv_from_inbox;
use super::reply_inbox;
use super::send::sys_ipc_send;

fn trace(pid: u32, label: &[u8], rc: i64) {
    if pid != 0x17 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-CALL] pid=");
    crate::sys::serial::trace_hex(pid as u64);
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

// Send-then-recv. Replies drain from the caller's dedicated reply
// inbox when the capsule spawn path assigned one; legacy callers fall
// back to `proc.<pid>`.
pub fn sys_ipc_call(
    ep: u64,
    req: u64,
    req_len: usize,
    resp: u64,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    let send_result = sys_ipc_send(ep, req, req_len);
    let pid = current_pid().unwrap_or(0);
    trace(pid, b"send", send_result);
    if send_result < 0 {
        return send_result;
    }
    let inbox = reply_inbox::for_pid(pid);
    let recv_result = recv_from_inbox(
        pid,
        &inbox,
        resp,
        resp_len,
        if timeout_ms == 0 { 5000 } else { timeout_ms },
    );
    trace(pid, b"recv", recv_result);
    recv_result
}
