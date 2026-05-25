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

use crate::ipc::nonos_inbox;
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_ACCES, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_TIMEDOUT,
};
use core::sync::atomic::{AtomicU32, Ordering};

static RECV_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 7 | 8 | 0x1b | 0x1c | 0x27)
}

fn trace(label: &[u8], pid: u32) {
    if !is_traced(pid) || RECV_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40 {
        return;
    }
    crate::sys::serial::print(b"[IPC-RECV] ");
    crate::sys::serial::println(label);
}

// Receive contract:
//   endpoint == 0  : default per-process inbox at `proc.<pid>`. No
//                    registry consult.
//   endpoint != 0  : named server inbox at `endpoint.<endpoint>`. The
//                    process must own the endpoint in the service
//                    registry. Non-owners denied with EACCES.
pub fn sys_ipc_recv(endpoint: u64, buf: u64, len: usize, timeout_ms: u64) -> i64 {
    if len == 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    let pid = current_pid().unwrap_or(0);
    trace(b"enter", pid);
    let inbox_name = if endpoint == 0 {
        alloc::format!("proc.{}", pid)
    } else {
        let target = alloc::format!("endpoint.{}", endpoint);
        match lookup_service(&target) {
            None => return ERRNO_NOENT,
            Some(ep) if ep.pid == pid => target,
            Some(_) => return ERRNO_ACCES,
        }
    };
    if !nonos_inbox::exists(&inbox_name) {
        trace(b"missing inbox", pid);
        return ERRNO_NOENT;
    }
    let start = crate::time::timestamp_millis();
    loop {
        if let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            trace(b"dequeue", pid);
            let copy_len = msg.data.len().min(len);
            if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
                return ERRNO_FAULT;
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
