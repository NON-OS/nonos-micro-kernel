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
use core::sync::atomic::AtomicBool;

use super::super::pending_reply;
use super::super::recv::recv_from_inbox;
use super::super::reply_inbox;
use super::super::send::sys_ipc_send;
use super::trace::trace;

static GPU_TRANSFER: AtomicBool = AtomicBool::new(false);
static GPU_SCANOUT: AtomicBool = AtomicBool::new(false);
static GPU_FLUSH: AtomicBool = AtomicBool::new(false);

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
    let inbox = reply_inbox::for_pid(pid);
    let endpoint = lookup_port(ep as u32);
    let endpoint_pid = endpoint.as_ref().map(|endpoint| endpoint.pid);
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
