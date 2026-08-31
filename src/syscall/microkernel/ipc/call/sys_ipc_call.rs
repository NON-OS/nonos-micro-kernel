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

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL};
use crate::{process::current_pid, services::registry::lookup_port};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::super::pending_reply;
use super::super::recv::recv_reply_correlated;
use super::super::reply_inbox;
use super::super::send::send_with_correlation;
use super::trace::trace;

static STARVED: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.call");

static GPU_TRANSFER: AtomicBool = AtomicBool::new(false);
static GPU_SCANOUT: AtomicBool = AtomicBool::new(false);
static GPU_FLUSH: AtomicBool = AtomicBool::new(false);

// Per-call correlation token: monotonic and never zero, so it can never collide
// with the correlation an attacker's `mk_ipc_send` produces (always 0) nor with
// another in-flight call. The reply must carry this exact value.
static CALL_TOKEN: AtomicU64 = AtomicU64::new(1);

fn next_call_token() -> u64 {
    let t = CALL_TOKEN.fetch_add(1, Ordering::Relaxed);
    if t == 0 {
        CALL_TOKEN.fetch_add(1, Ordering::Relaxed)
    } else {
        t
    }
}

pub fn sys_ipc_call(
    ep: u64,
    req: u64,
    req_len: usize,
    resp: u64,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    let pid = current_pid().unwrap_or(0);
    if resp_len == 0 || resp_len > crate::ipc::channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(resp, resp_len).is_err() {
        return ERRNO_FAULT;
    }
    // This call's correlation token, registered with the pending reply keyed by
    // this caller's inbox. Both reply paths read it back from that same entry:
    // the redirect path (a service replying via `mk_ipc_send` to its fixed reply
    // endpoint) and the direct `mk_ipc_reply` path both stamp the reply with it.
    // The genuine reply carries this token; a forged injection can only carry 0.
    let token = next_call_token();
    let inbox = reply_inbox::for_pid(pid);
    let endpoint = lookup_port(ep as u32);
    let endpoint_pid = endpoint.as_ref().map(|endpoint| endpoint.pid);
    if let Some(server_pid) = endpoint_pid {
        if !pending_reply::push(server_pid, pid, inbox.clone(), token) {
            return ERRNO_BUSY;
        }
    }
    let send_result = send_with_correlation(ep, req, req_len, token);
    trace(pid, b"send", send_result);
    if send_result < 0 {
        if let Some(server_pid) = endpoint_pid {
            // Strip exactly the entry this call pushed: the newest match. An
            // older match can be a reply still owed from a timed-out call, and
            // removing it shifts the server's FIFO onto the wrong callers.
            pending_reply::remove_latest(server_pid, &inbox);
        }
        return send_result;
    }
    let timeout = if timeout_ms == 0 { 5000 } else { timeout_ms };
    let recv_result = recv_reply_correlated(pid, &inbox, resp, resp_len, timeout, token);
    if recv_result < 0 {
        // The pending entry is NOT removed on a timeout. The server received
        // this request and will still reply to it; the redirect pairs replies
        // to callers strictly by FIFO position, so consuming an entry out of
        // order shifts every later reply onto the wrong caller, stamped with
        // that caller's own token. One timeout then desyncs the server's whole
        // reply stream and the misdeliveries cause further timeouts. Left in
        // place, the entry is popped in order and the late reply lands on this
        // (no longer waiting) inbox, where the correlation check discards it.
        // Entries are only removed when the send itself failed, where the
        // server never saw a request, and by clear_pid when either side dies.
        //
        // A negative receive after a successful send is a served call that got
        // no answer: the server is wedged or the reply path lost the message.
        // Refusals log at the gates, so this line is specifically starvation.
        if let Some(endpoint) = endpoint.as_ref() {
            STARVED.starved(&endpoint.name, pid, recv_result, endpoint.pid);
        }
    }
    if recv_result >= 24 && req_len >= 20 {
        if let Some(endpoint) = endpoint.as_ref() {
            let mut hdr = [0u8; 20];
            let mut status = [0u8; 4];
            let ok_hdr = crate::usercopy::copy_from_user(req, &mut hdr).is_ok();
            let ok_status = crate::usercopy::copy_from_user(resp + 20, &mut status).is_ok();
            if endpoint.name.as_bytes() == b"driver.virtio_gpu0" && ok_hdr && ok_status {
                let magic = u32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
                let op = u16::from_le_bytes([hdr[6], hdr[7]]);
                if magic == 0x4E56_4750 && u32::from_le_bytes(status) == 0 {
                    if op == 0x0008 {
                        crate::sys::bench::mark_once(&GPU_TRANSFER, b"virtio_gpu_transfer_first");
                    }
                    if op == 0x0009 {
                        crate::sys::bench::mark_once(&GPU_SCANOUT, b"virtio_gpu_scanout_first");
                    }
                    if op == 0x000A {
                        crate::sys::bench::mark_once(&GPU_FLUSH, b"virtio_gpu_flush_first");
                    }
                }
            }
        }
    }
    trace(pid, b"recv", recv_result);
    recv_result
}
