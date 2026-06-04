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
use crate::services::registry::lookup_port;
use crate::syscall::microkernel::errnos::ERRNO_BUSY;

use super::super::pending_reply;
use super::super::recv::recv_from_inbox;
use super::super::reply_inbox;
use super::super::send::sys_ipc_send;
use super::trace::trace;

pub fn sys_ipc_call(
    ep: u64,
    req: u64,
    req_len: usize,
    resp: u64,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    let pid = current_pid().unwrap_or(0);
    let inbox = reply_inbox::for_pid(pid);
    let endpoint_pid = lookup_port(ep as u32).map(|endpoint| endpoint.pid);
    if let Some(server_pid) = endpoint_pid {
        if !pending_reply::push(server_pid, inbox.clone()) {
            return ERRNO_BUSY;
        }
    }
    let send_result = sys_ipc_send(ep, req, req_len);
    trace(pid, b"send", send_result);
    if send_result < 0 {
        if let Some(server_pid) = endpoint_pid {
            pending_reply::remove(server_pid, &inbox);
        }
        return send_result;
    }
    let timeout = if timeout_ms == 0 { 5000 } else { timeout_ms };
    let recv_result = recv_from_inbox(pid, &inbox, resp, resp_len, timeout);
    if recv_result < 0 {
        if let Some(server_pid) = endpoint_pid {
            pending_reply::remove(server_pid, &inbox);
        }
    }
    trace(pid, b"recv", recv_result);
    recv_result
}
