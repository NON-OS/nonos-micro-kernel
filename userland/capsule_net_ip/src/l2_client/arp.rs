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

use nonos_libc::mk_ipc_call;

use super::header::{parse_response, write_request};
use super::seq;
use super::wire::{L2_HDR_LEN, OP_ARP_RESOLVE};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArpError {
    SendFailed,
    BadResponse,
    NoNeighbour,
    NoLink,
    Other(u16),
}

// Ask `net.l2` to resolve `target` IPv4 to a MAC. On miss the L2
// capsule emits an ARP request and returns NoNeighbour; the IP
// caller backs off and retries.
pub fn resolve(l2_port: u32, target: [u8; 4]) -> Result<[u8; 6], ArpError> {
    let total = L2_HDR_LEN + 4;
    let mut req = [0u8; L2_HDR_LEN + 4];
    let rid = seq::next();
    write_request(&mut req, OP_ARP_RESOLVE, rid, 4);
    req[L2_HDR_LEN..total].copy_from_slice(&target);
    let mut resp = [0u8; L2_HDR_LEN + 6];
    let n = mk_ipc_call(l2_port as u64, req.as_ptr(), total, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(ArpError::SendFailed);
    }
    let (op, errno, _, plen) = parse_response(&resp).ok_or(ArpError::BadResponse)?;
    if op != OP_ARP_RESOLVE {
        return Err(ArpError::BadResponse);
    }
    if errno == 6 {
        return Err(ArpError::NoNeighbour);
    }
    if errno == 5 {
        return Err(ArpError::NoLink);
    }
    if errno != 0 {
        return Err(ArpError::Other(errno));
    }
    if plen as usize != 6 {
        return Err(ArpError::BadResponse);
    }
    let mut mac = [0u8; 6];
    mac.copy_from_slice(&resp[L2_HDR_LEN..L2_HDR_LEN + 6]);
    Ok(mac)
}
