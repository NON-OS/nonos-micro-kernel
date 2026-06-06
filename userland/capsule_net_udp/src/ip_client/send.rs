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

use nonos_libc::mk_ipc_call;

use super::header::{parse_response, write_request};
use super::seq;
use super::wire::{IP_HDR_LEN, IP_PROTO_UDP, OP_SEND_PACKET};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SendError {
    SendFailed,
    BadResponse,
    Refused(u16),
}

// Ship one UDP segment down to `net.ip` for routing+TX. Body
// is the 4-byte dst IPv4 + 1-byte protocol (17) + UDP segment.
pub fn send_segment(ip_port: u32, dst: [u8; 4], segment: &[u8]) -> Result<(), SendError> {
    let body_len = 4 + 1 + segment.len();
    let total = IP_HDR_LEN + body_len;
    let mut req = vec![0u8; total];
    let rid = seq::next();
    write_request(&mut req, OP_SEND_PACKET, rid, body_len as u32);
    let mut cur = IP_HDR_LEN;
    req[cur..cur + 4].copy_from_slice(&dst);
    cur += 4;
    req[cur] = IP_PROTO_UDP;
    cur += 1;
    req[cur..cur + segment.len()].copy_from_slice(segment);
    let mut resp = [0u8; IP_HDR_LEN];
    let n = mk_ipc_call(ip_port as u64, req.as_ptr(), total, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(SendError::SendFailed);
    }
    let (op, errno, _, _) = parse_response(&resp).ok_or(SendError::BadResponse)?;
    if op != OP_SEND_PACKET {
        return Err(SendError::BadResponse);
    }
    if errno != 0 {
        return Err(SendError::Refused(errno));
    }
    Ok(())
}
