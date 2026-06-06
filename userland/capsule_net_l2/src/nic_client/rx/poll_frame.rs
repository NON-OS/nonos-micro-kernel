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

use super::super::header::write_request;
use super::super::seq;
use super::super::wire::{NIC_HDR_LEN, OP_RX_PACKET};
use super::constants::RESP_CAP;
use super::error::RxError;
use super::parse_payload::parse_payload;

pub fn poll_frame(nic_port: u32) -> Result<alloc::vec::Vec<u8>, RxError> {
    let mut req = [0u8; NIC_HDR_LEN];
    let rid = seq::next();
    if write_request(&mut req, OP_RX_PACKET, rid, 0).is_none() {
        return Err(RxError::BadResponse);
    }
    let mut resp = vec![0u8; RESP_CAP];
    let n = mk_ipc_call(nic_port as u64, req.as_ptr(), NIC_HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(RxError::SendFailed);
    }
    let got = n as usize;
    if got > resp.len() {
        return Err(RxError::BadResponse);
    }
    parse_payload(&resp[..got])
}
