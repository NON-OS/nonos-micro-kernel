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

use crate::arp::{build_request, Iface};
use crate::nic_client::send_frame as nic_send;
use crate::protocol::{
    Request, E_BAD_LEN, E_NO_LINK, E_NO_NEIGHBOUR, E_OK, HDR_LEN, OP_ARP_RESOLVE,
};
use crate::server::respond::respond;
use crate::state::STATE;

// Body layout (request): 4 bytes target IPv4. Response (success):
// 6 bytes MAC. On cache miss the L2 emits a broadcast ARP request
// and returns E_NO_NEIGHBOUR so the caller can retry.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != 4 {
        let _ = respond(sender_pid, OP_ARP_RESOLVE, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    let mut target = [0u8; 4];
    target.copy_from_slice(body);
    if let Some(mac) = STATE.arp.lock().lookup(&target) {
        tx[HDR_LEN..HDR_LEN + 6].copy_from_slice(&mac);
        let _ = respond(sender_pid, OP_ARP_RESOLVE, E_OK, req.request_id, 6, tx);
        return;
    }
    let nic = STATE.nic_port();
    if nic == 0 {
        let _ = respond(sender_pid, OP_ARP_RESOLVE, E_NO_LINK, req.request_id, 0, tx);
        return;
    }
    let iface = Iface { mac: *STATE.mac.lock(), ipv4: *STATE.ipv4.lock() };
    STATE.arp.lock().note_pending(target);
    let req_frame = build_request(&iface, target);
    let _ = nic_send(nic, &req_frame);
    let _ = respond(sender_pid, OP_ARP_RESOLVE, E_NO_NEIGHBOUR, req.request_id, 0, tx);
}
