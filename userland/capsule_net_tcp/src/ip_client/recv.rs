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
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

use super::header::{parse_response, write_request};
use super::seq;
use super::wire::{IP_HDR_LEN, IP_PROTO_TCP, OP_POLL_PACKET};

#[derive(Clone, PartialEq, Eq)]
pub struct IpPacket {
    pub src: [u8; 4],
    pub dst: [u8; 4],
    pub segment: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RecvError {
    Send,
    BadResponse,
    Empty,
    Other(u16),
}

pub fn poll_segment(ip_port: u32) -> Result<IpPacket, RecvError> {
    let mut req = [0u8; IP_HDR_LEN + 1];
    write_request(&mut req, OP_POLL_PACKET, seq::next(), 1);
    req[IP_HDR_LEN] = IP_PROTO_TCP;
    let mut resp = vec![0u8; IP_HDR_LEN + 9 + 1500];
    let n = mk_ipc_call(ip_port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(RecvError::Send);
    }
    let (op, errno, _, plen) = parse_response(&resp).ok_or(RecvError::BadResponse)?;
    if op != OP_POLL_PACKET {
        return Err(RecvError::BadResponse);
    }
    parse_packet(&resp, errno, plen)
}

fn parse_packet(resp: &[u8], errno: u16, plen: u32) -> Result<IpPacket, RecvError> {
    if errno == 10 {
        return Err(RecvError::Empty);
    }
    if errno != 0 {
        return Err(RecvError::Other(errno));
    }
    let end = IP_HDR_LEN + plen as usize;
    if plen < 9 || end > resp.len() || resp[IP_HDR_LEN + 8] != IP_PROTO_TCP {
        return Err(RecvError::BadResponse);
    }
    let mut src = [0u8; 4];
    let mut dst = [0u8; 4];
    src.copy_from_slice(&resp[IP_HDR_LEN..IP_HDR_LEN + 4]);
    dst.copy_from_slice(&resp[IP_HDR_LEN + 4..IP_HDR_LEN + 8]);
    Ok(IpPacket { src, dst, segment: resp[IP_HDR_LEN + 9..end].to_vec() })
}
