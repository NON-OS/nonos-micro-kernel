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
use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox;
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
    let target = resolve_send_target(endpoint);
    match redirect_reply(pid, &target) {
        // A capsule replying to a request another capsule made with mk_ipc_call:
        // hand the bytes to that caller's private inbox stamped with the token
        // it waits on, and wake it since a reply inbox has no owner the router
        // would wake on its own.
        Redirect::ToCaller { caller_inbox, caller_pid, token } => {
            if !super::send_caps::caller_satisfies_endpoint(endpoint, &caller_inbox) {
                return ERRNO_PERM;
            }
            trace(pid, endpoint, &caller_inbox, len);
            // Enqueue straight into the caller's reply inbox, the exact inbox its
            // blocked mk_ipc_call is draining, stamped with the token that call
            // filters on. Going back through the service resolver would look the
            // reply inbox up as a service and re-resolve it to the caller's proc
            // inbox (the caller adopted this endpoint), where the caller's serve
            // loop eats the reply and the call times out. That is why every
            // userland call into a kernel-reply service (crypto, the block
            // device, vfs) hung: the answer was delivered to the wrong inbox.
            match IpcMessage::new(&alloc::format!("proc.{}", pid), &caller_inbox, &data) {
                Ok(mut msg) => {
                    msg.correlation = token;
                    match nonos_inbox::try_enqueue_strict(&caller_inbox, msg) {
                        Ok(()) => {
                            crate::sched::wake_process(caller_pid);
                            0
                        }
                        Err(_) => ERRNO_FAULT,
                    }
                }
                Err(_) => ERRNO_FAULT,
            }
        }
        // A capsule replying to a kernel-mediated round trip (crypto_pool,
        // entropy, vfs, the block device). The kernel drains this exact reply
        // inbox, so put the bytes there directly. Routing it as addressed would
        // send it to proc.<self>, where the serve loop reads its own reply as a
        // request and self-mails a core to death; that is the loop the old drop
        // guarded, and dropping instead stranded every kernel round trip.
        Redirect::ToReplyInbox => match IpcMessage::new(&alloc::format!("proc.{}", pid), &target, &data) {
            Ok(msg) => {
                let _ = nonos_inbox::try_enqueue_strict(&target, msg);
                0
            }
            Err(_) => 0,
        },
        // Any other send goes to its addressed target with its own correlation
        // (0 for sys_ipc_send, all a forged reply injection can carry).
        Redirect::AsAddressed => {
            if !super::send_caps::caller_satisfies_endpoint(endpoint, &target) {
                return ERRNO_PERM;
            }
            trace(pid, endpoint, &target, len);
            match kernel_route_ipc_corr(pid, &target, &data, correlation) {
                Ok(()) => 0,
                Err(e) => e as i64,
            }
        }
    }
}

enum Redirect {
    ToCaller { caller_inbox: alloc::string::String, caller_pid: u32, token: u64 },
    ToReplyInbox,
    AsAddressed,
}

// Classify a send. A capsule sending to its own fixed reply endpoint is
// answering a request, and which request decides where the bytes go: a pending
// mk_ipc_call caller takes them (ToCaller), otherwise the kernel is the one
// waiting on this inbox for a round trip it drove, so they stay here for the
// drain (ToReplyInbox). Every other send is addressed as written (AsAddressed).
fn redirect_reply(sender_pid: u32, target: &str) -> Redirect {
    let own_reply = crate::process::get_process(sender_pid).and_then(|p| p.reply_inbox());
    if own_reply == Some(target) {
        if let Some((caller_pid, caller_inbox, token)) = super::pending_reply::pop(sender_pid) {
            return Redirect::ToCaller { caller_inbox, caller_pid, token };
        }
        return Redirect::ToReplyInbox;
    }
    Redirect::AsAddressed
}

fn resolve_send_target(endpoint: u64) -> alloc::string::String {
    let numeric = alloc::format!("endpoint.{}", endpoint);
    if lookup_service(&numeric).is_some() {
        return numeric;
    }
    lookup_port(endpoint as u32).map(|ep| ep.name).unwrap_or(numeric)
}
