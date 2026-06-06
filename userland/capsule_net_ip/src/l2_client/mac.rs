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
use super::wire::{L2_HDR_LEN, OP_GET_MAC};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MacError {
    SendFailed,
    BadResponse,
    L2Refused,
}

pub fn read_mac(l2_port: u32) -> Result<[u8; 6], MacError> {
    let mut req = [0u8; L2_HDR_LEN];
    let rid = seq::next();
    write_request(&mut req, OP_GET_MAC, rid, 0);
    let mut resp = [0u8; L2_HDR_LEN + 6];
    let n = mk_ipc_call(l2_port as u64, req.as_ptr(), L2_HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(MacError::SendFailed);
    }
    let (op, errno, _, plen) = parse_response(&resp).ok_or(MacError::BadResponse)?;
    if op != OP_GET_MAC || errno != 0 {
        return Err(MacError::L2Refused);
    }
    if plen as usize != 6 {
        return Err(MacError::BadResponse);
    }
    let mut mac = [0u8; 6];
    mac.copy_from_slice(&resp[L2_HDR_LEN..L2_HDR_LEN + 6]);
    Ok(mac)
}
