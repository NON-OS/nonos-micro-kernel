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
use super::wire::{IP_HDR_LEN, IP_PROTO_TCP, OP_SEND_PACKET};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SendError {
    Send,
    BadResponse,
    Refused(u16),
}

pub fn send_segment(ip_port: u32, dst: [u8; 4], segment: &[u8]) -> Result<(), SendError> {
    let body_len = 5 + segment.len();
    let mut req = vec![0u8; IP_HDR_LEN + body_len];
    write_request(&mut req, OP_SEND_PACKET, seq::next(), body_len as u32);
    req[IP_HDR_LEN..IP_HDR_LEN + 4].copy_from_slice(&dst);
    req[IP_HDR_LEN + 4] = IP_PROTO_TCP;
    req[IP_HDR_LEN + 5..].copy_from_slice(segment);
    let mut resp = [0u8; IP_HDR_LEN];
    let n = mk_ipc_call(ip_port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(SendError::Send);
    }
    let (op, errno, _, _) = parse_response(&resp).ok_or(SendError::BadResponse)?;
    if op != OP_SEND_PACKET {
        return Err(SendError::BadResponse);
    }
    if errno == 0 {
        Ok(())
    } else {
        Err(SendError::Refused(errno))
    }
}
