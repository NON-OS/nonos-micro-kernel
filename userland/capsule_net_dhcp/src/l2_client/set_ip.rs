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
use super::wire::{L2_HDR_LEN, OP_SET_IP};

// Body layout matches `capsule_net_l2`'s set_ip handler: 4-byte IPv4.
const BODY_LEN: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetIpError {
    SendFailed,
    BadResponse,
    Refused(u16),
}

// Hand the leased IPv4 to L2 so it can source ARP from a real
// address and answer ARP requests for the host. Best-effort: the
// lease itself is already installed in `net.ip` before this runs.
pub fn set_ip(l2_port: u32, ipv4: [u8; 4]) -> Result<(), SetIpError> {
    let total = L2_HDR_LEN + BODY_LEN;
    let mut req = [0u8; L2_HDR_LEN + BODY_LEN];
    let rid = seq::next();
    write_request(&mut req, OP_SET_IP, rid, BODY_LEN as u32);
    req[L2_HDR_LEN..total].copy_from_slice(&ipv4);
    let mut resp = [0u8; L2_HDR_LEN];
    let n = mk_ipc_call(l2_port as u64, req.as_ptr(), total, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(SetIpError::SendFailed);
    }
    let (op, errno, _, _) = parse_response(&resp).ok_or(SetIpError::BadResponse)?;
    if op != OP_SET_IP {
        return Err(SetIpError::BadResponse);
    }
    if errno != 0 {
        return Err(SetIpError::Refused(errno));
    }
    Ok(())
}
