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
use super::wire::{NIC_HDR_LEN, OP_MAC_ADDRESS};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MacError {
    SendFailed,
    BadResponse,
    BadLength,
}

pub fn read_mac(nic_port: u32) -> Result<[u8; 6], MacError> {
    let mut req = [0u8; NIC_HDR_LEN];
    let rid = seq::next();
    if write_request(&mut req, OP_MAC_ADDRESS, rid, 0).is_none() {
        return Err(MacError::BadResponse);
    }
    let mut resp = [0u8; NIC_HDR_LEN + 4 + 6];
    let n = mk_ipc_call(nic_port as u64, req.as_ptr(), NIC_HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(MacError::SendFailed);
    }
    let got = n as usize;
    if got > resp.len() {
        return Err(MacError::BadResponse);
    }
    let view = &resp[..got];
    let (op, _rid_back, plen) = parse_response(view).ok_or(MacError::BadResponse)?;
    if op != OP_MAC_ADDRESS || plen as usize != 4 + 6 {
        return Err(MacError::BadLength);
    }
    if view.len() < NIC_HDR_LEN + 4 + 6 {
        return Err(MacError::BadLength);
    }
    let mac_start = NIC_HDR_LEN + 4;
    let mut mac = [0u8; 6];
    mac.copy_from_slice(&view[mac_start..mac_start + 6]);
    Ok(mac)
}
