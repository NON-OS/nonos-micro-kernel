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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::ipc::kernel_ipc::kernel_route_ipc_corr;
use crate::process::current_pid;
use crate::services::registry::{lookup_port, lookup_service};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};

static SEND_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 0x18 | 0x1a | 0x1b)
}

fn trace(pid: u32, endpoint: u64, target: &str, len: usize) {
    if !is_traced(pid) || SEND_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 48 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-SEND] pid=");
    crate::sys::serial::trace_hex(pid as u64);
    crate::sys::serial::trace(b" ep=");
    crate::sys::serial::trace_hex(endpoint);
    crate::sys::serial::trace(b" len=");
    crate::sys::serial::trace_dec(len as u64);
    crate::sys::serial::trace(b" target=");
    crate::sys::serial::trace(target.as_bytes());
    crate::sys::serial::traceln(b"");
}

pub fn sys_ipc_send(endpoint: u64, buf: u64, len: usize) -> i64 {
    send_with_correlation(endpoint, buf, len, 0)
}

pub(super) fn send_with_correlation(endpoint: u64, buf: u64, len: usize, correlation: u64) -> i64 {
    // Reject oversize before allocating: len is validated against the 64 MiB
    // usercopy ceiling downstream, but the message itself is capped at 1 MiB,
    // so bound it here to avoid a capsule forcing huge transient allocations.
    if len == 0 || len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_read(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    let mut data = alloc::vec![0u8; len];
    if crate::usercopy::copy_from_user(buf, &mut data).is_err() {
        return ERRNO_FAULT;
    }
    let pid = current_pid().unwrap_or(0);
    // An undeliverable reply reports success to the sender: from the server's
    // side the reply was handed off, and there is nobody left to hand it to.
    let Some((target, caller, redirect_token)) = redirect_reply(pid, resolve_send_target(endpoint))
    else {
        return 0;
    };
    if !super::send_caps::caller_satisfies_endpoint(endpoint, &target) {
        return ERRNO_PERM;
    }
    // A redirected reply carries the caller's per-call token so the caller can
    // match it; a plain send keeps its own correlation (0 for `sys_ipc_send`,
    // which is what a forged reply-injection is limited to).
    let correlation = redirect_token.unwrap_or(correlation);
    trace(pid, endpoint, &target, len);
    match kernel_route_ipc_corr(pid, &target, &data, correlation) {
        Ok(()) => {
            if let Some(caller_pid) = caller {
                crate::sched::wake_process(caller_pid);
            }
            0
        }
        Err(e) => e as i64,
    }
}

// When a service replies to its own fixed reply endpoint, hand the message to
// the matching `mk_ipc_call` caller's private inbox instead, and surface that
// caller's correlation token so the reply can be stamped with it. A non-reply
// send is left as-is with no token. The caller pid rides along because a
// kernel-owned reply inbox has no process owner for the router to wake, so the
// blocked caller would otherwise sleep out its whole `mk_ipc_call` timeout.
//
// A reply with NO pending caller returns `None`: it is undeliverable. Since
// endpoint adoption made a service own its reply endpoint, delivering it "as
// addressed" routes it into the sender's own inbox, where the serve loop reads
// it as a request and replies to the reply — a self-mail loop that pins a core
// within seconds of one stray message. Dropping it is the only safe answer.
fn redirect_reply(
    sender_pid: u32,
    target: alloc::string::String,
) -> Option<(alloc::string::String, Option<u32>, Option<u64>)> {
    let own_reply = crate::process::get_process(sender_pid).and_then(|p| p.reply_inbox());
    if own_reply == Some(target.as_str()) {
        if let Some((caller_pid, caller_inbox, token)) = super::pending_reply::pop(sender_pid) {
            return Some((caller_inbox, Some(caller_pid), Some(token)));
        }
        BOOMERANG.reject(&target, "reply with no caller, dropped", sender_pid);
        return None;
    }
    Some((target, None, None))
}

static BOOMERANG: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.reply");

fn resolve_send_target(endpoint: u64) -> alloc::string::String {
    let numeric = alloc::format!("endpoint.{}", endpoint);
    if lookup_service(&numeric).is_some() {
        return numeric;
    }
    lookup_port(endpoint as u32).map(|ep| ep.name).unwrap_or(numeric)
}
