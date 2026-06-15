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

use alloc::vec;

use nonos_libc::mk_ipc_recv_from;

use crate::protocol::{
    parse, E_BAD_OP, HDR_LEN, IPC_PAYLOAD_MAX, OP_ARP_RESOLVE, OP_GET_LINK, OP_GET_MAC,
    OP_HEALTHCHECK, OP_POLL_FRAME, OP_SEND_FRAME, OP_SET_IP,
};

use super::handlers;
use super::respond::respond_status_only;

const SERVICE_INBOX: u64 = 0; // capsule's own per-process inbox

pub fn run() -> ! {
    let rx_len = HDR_LEN + IPC_PAYLOAD_MAX;
    let tx_len = HDR_LEN + IPC_PAYLOAD_MAX;
    let mut rx = vec![0u8; rx_len];
    let mut tx = vec![0u8; tx_len];
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx_len, 0, &mut sender_pid);
        if n <= 0 || sender_pid == 0 {
            continue;
        }
        let len = n as usize;
        let Ok((req, body)) = parse(&rx[..len]) else { continue };
        match req.op {
            OP_HEALTHCHECK => handlers::health::handle(sender_pid, &req, &mut tx),
            OP_GET_MAC => handlers::get_mac::handle(sender_pid, &req, &mut tx),
            OP_GET_LINK => handlers::get_link::handle(sender_pid, &req, &mut tx),
            OP_SEND_FRAME => handlers::send_frame::handle(sender_pid, &req, body, &mut tx),
            OP_POLL_FRAME => handlers::poll_frame::handle(sender_pid, &req, &mut tx),
            OP_ARP_RESOLVE => handlers::arp_resolve::handle(sender_pid, &req, body, &mut tx),
            OP_SET_IP => handlers::set_ip::handle(sender_pid, &req, body, &mut tx),
            _ => {
                let _ = respond_status_only(sender_pid, req.op, E_BAD_OP, req.request_id, &mut tx);
            }
        }
    }
}
