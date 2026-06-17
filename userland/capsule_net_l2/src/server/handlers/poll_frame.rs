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

use crate::ingress::observe;
use crate::nic_client::{poll_frame as nic_poll, RxError};
use crate::protocol::{Request, E_NO_LINK, E_OK, E_PERM, E_RX_EMPTY, HDR_LEN, OP_POLL_FRAME};
use crate::server::authz::authorized_any;
use crate::server::respond::respond;
use crate::state::STATE;

// Body (response, success): the raw ethernet frame the NIC handed
// us. Before delivering, we run the frame through the L2 ingress
// observer so ARP replies seed the neighbour cache without the
// upstream consumer having to know what an ARP packet looks like.
pub fn handle(sender_pid: u32, req: &Request, tx: &mut [u8]) {
    if !authorized_any(sender_pid, &[b"net.ip", b"net.dhcp.client"]) {
        let _ = respond(sender_pid, OP_POLL_FRAME, E_PERM, req.request_id, 0, tx);
        return;
    }
    let nic = STATE.nic_port();
    if nic == 0 {
        let _ = respond(sender_pid, OP_POLL_FRAME, E_NO_LINK, req.request_id, 0, tx);
        return;
    }
    match nic_poll(nic) {
        Ok(frame) => deliver(sender_pid, req, &frame, tx),
        Err(RxError::Empty) => {
            let _ = respond(sender_pid, OP_POLL_FRAME, E_RX_EMPTY, req.request_id, 0, tx);
        }
        Err(_) => {
            let _ = respond(sender_pid, OP_POLL_FRAME, E_NO_LINK, req.request_id, 0, tx);
        }
    }
}

fn deliver(sender_pid: u32, req: &Request, frame: &[u8], tx: &mut [u8]) {
    observe(frame);
    if frame.len() <= tx.len() - HDR_LEN {
        tx[HDR_LEN..HDR_LEN + frame.len()].copy_from_slice(frame);
        let _ = respond(sender_pid, OP_POLL_FRAME, E_OK, req.request_id, frame.len() as u32, tx);
    } else {
        let _ = respond(sender_pid, OP_POLL_FRAME, E_RX_EMPTY, req.request_id, 0, tx);
    }
}
