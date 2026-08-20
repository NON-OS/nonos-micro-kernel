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
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_TIMEDOUT};
use core::sync::atomic::{AtomicU32, Ordering};

use super::inbox_name::resolve_for_recv;

static RECV_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 7 | 8 | 0x18 | 0x1a | 0x1b | 0x1c | 0x27)
}

fn trace(label: &[u8], pid: u32) {
    if !is_traced(pid) || RECV_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 80 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-RECV] pid=");
    crate::sys::serial::trace_hex(pid as u64);
    crate::sys::serial::trace(b" ");
    crate::sys::serial::traceln(label);
}

pub fn sys_ipc_recv(endpoint: u64, buf: u64, len: usize, timeout_ms: u64) -> i64 {
    if len == 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    let pid = current_pid().unwrap_or(0);
    trace(b"enter", pid);
    let inbox_name = match resolve_for_recv(endpoint, pid) {
        Ok(name) => name,
        Err(e) => return e,
    };
    recv_from_inbox(pid, &inbox_name, buf, len, timeout_ms)
}

/// Receive the reply to an outstanding `mk_ipc_call`, accepting ONLY a message
/// whose correlation equals the token the call sent. Every real reply carries
/// that token, because `mk_ipc_reply` stamps it from the correlation the server
/// captured on the request. A message in the caller's reply inbox with any other
/// correlation is a forgery — a hostile capsule can `mk_ipc_send` into another
/// capsule's reply inbox, but `sys_ipc_send` hardcodes correlation 0, so it can
/// never match a (nonzero) token. Such messages are discarded, not delivered,
/// closing the reply-injection / confused-deputy hole.
pub(super) fn recv_reply_correlated(
    pid: u32,
    inbox_name: &str,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    want: u64,
) -> i64 {
    if !nonos_inbox::exists(&inbox_name) {
        return ERRNO_NOENT;
    }
    let start = crate::time::timestamp_millis();
    loop {
        // Token before the drain, for the same lost-wakeup reason as
        // `recv_from_inbox`: a reply landing between an empty drain and the
        // transition to Sleeping has already spent its wake.
        let token = crate::sched::wake_token(pid);
        // Drain everything queued; deliver the matching reply, drop the rest
        // (forged, stale, or a duplicate from a previous call).
        while let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            if msg.correlation == want {
                let copy_len = msg.data.len().min(len);
                if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
                    return ERRNO_FAULT;
                }
                return copy_len as i64;
            }
        }
        let elapsed = crate::time::timestamp_millis().saturating_sub(start);
        if timeout_ms > 0 && elapsed >= timeout_ms {
            return ERRNO_TIMEDOUT;
        }
        let deadline = if timeout_ms == 0 { u64::MAX } else { start.saturating_add(timeout_ms) };
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        crate::sched::yield_now();
    }
}

static NO_INBOX: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.recv");
// The first 64 dequeues systemwide, then sampled: enough to show which inboxes
// actually drain during bring-up without narrating every message after.
static FIRST_DRAIN: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.drain");

pub(super) fn recv_from_inbox(
    pid: u32,
    inbox_name: &str,
    buf: u64,
    len: usize,
    timeout_ms: u64,
) -> i64 {
    if !nonos_inbox::exists(&inbox_name) {
        // A server receiving on an inbox that does not exist is unreachable
        // forever and every caller reads as starvation. Loud, not trace-gated.
        NO_INBOX.reject(inbox_name, "missing inbox", pid);
        return ERRNO_NOENT;
    }
    let start = crate::time::timestamp_millis();
    loop {
        // The wake token is read before the queue check. A delivery bumps the
        // token and then enqueues under the same route, so a wake landing
        // after this read blocks the sleep below, and one landing before it
        // left its message where the very next check finds it. Without the
        // token there is a gap between the empty check and the transition to
        // Sleeping, and a message arriving in that gap has spent its only
        // wake on a still-Running process: the receiver then sleeps on a
        // queue that has data, forever if the timeout is infinite.
        let token = crate::sched::wake_token(pid);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            // Logged as sender -> receiver so a flooded inbox names its
            // flooder, not just itself.
            FIRST_DRAIN.note(&msg.from, pid as u64);
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
        let deadline = if timeout_ms == 0 { u64::MAX } else { start.saturating_add(timeout_ms) };
        crate::sched::sleep_until_unless_woken(pid, deadline, token);
        if let Some(msg) = nonos_inbox::try_dequeue_existing(&inbox_name) {
            trace(b"dequeue", pid);
            let copy_len = msg.data.len().min(len);
            if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
                return ERRNO_FAULT;
            }
            return copy_len as i64;
        }
        trace(b"before yield", pid);
        crate::sched::yield_now();
        trace(b"after yield", pid);
    }
}
