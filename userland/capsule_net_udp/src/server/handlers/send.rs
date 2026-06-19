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

use crate::ip_client::{read_ipv4, send_segment, SendError};
use crate::protocol::{E_BAD_LEN, E_NO_IP_LINK, E_NO_PORT, E_OK, OP_SEND, UDP_PAYLOAD_MAX};
use crate::server::parse_req::Request;
use crate::server::respond::respond;
use crate::state::STATE;
use crate::udp::{build, BuildRequest, HDR_LEN as UDP_HDR_LEN};

// Body: 2 src_port + 4 dst IPv4 + 2 dst_port + payload.
pub fn handle(sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() < 8 {
        let _ = respond(sender_pid, OP_SEND, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    let src_port = u16::from_le_bytes([body[0], body[1]]);
    let mut dst = [0u8; 4];
    dst.copy_from_slice(&body[2..6]);
    let dst_port = u16::from_le_bytes([body[6], body[7]]);
    let payload = &body[8..];
    if payload.len() > UDP_PAYLOAD_MAX {
        let _ = respond(sender_pid, OP_SEND, E_BAD_LEN, req.request_id, 0, tx);
        return;
    }
    if STATE.binds.lock().find_owned_mut(sender_pid, src_port).is_none() {
        let _ = respond(sender_pid, OP_SEND, E_NO_PORT, req.request_id, 0, tx);
        return;
    }
    let ip_port = STATE.ip_port();
    if ip_port == 0 {
        let _ = respond(sender_pid, OP_SEND, E_NO_IP_LINK, req.request_id, 0, tx);
        return;
    }
    let src = source_ipv4(ip_port);
    let mut seg = vec![0u8; UDP_HDR_LEN + payload.len()];
    let n = match build(&BuildRequest { src, dst, src_port, dst_port, payload }, &mut seg) {
        Ok(n) => n,
        Err(_) => {
            let _ = respond(sender_pid, OP_SEND, E_BAD_LEN, req.request_id, 0, tx);
            return;
        }
    };
    let errno = match send_segment(ip_port, dst, &seg[..n]) {
        Ok(()) => E_OK,
        Err(SendError::SendFailed) | Err(SendError::BadResponse) | Err(SendError::Refused(_)) => {
            E_NO_IP_LINK
        }
    };
    let _ = respond(sender_pid, OP_SEND, errno, req.request_id, 0, tx);
}

fn source_ipv4(ip_port: u32) -> [u8; 4] {
    let cached = *STATE.local_ipv4.lock();
    if cached != [0; 4] {
        return cached;
    }
    match read_ipv4(ip_port) {
        Ok(addr) => {
            *STATE.local_ipv4.lock() = addr;
            addr
        }
        Err(_) => cached,
    }
}
