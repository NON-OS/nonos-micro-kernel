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
use super::wire::{NIC_HDR_LEN, OP_RX_PACKET};

pub const MAX_FRAME: usize = 1514;
const PREFIX_LEN: usize = 4 + 4; // status + length
const RESP_CAP: usize = NIC_HDR_LEN + PREFIX_LEN + 12 /* virtio_net header */ + MAX_FRAME;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxError {
    SendFailed,
    BadResponse,
    Empty,
}

// Poll the NIC for one frame. Returns the frame bytes on success
// (caller-owned, freshly allocated) or `Empty` when the ring has
// nothing right now — the main loop yields and tries again.
pub fn poll_frame(nic_port: u32) -> Result<alloc::vec::Vec<u8>, RxError> {
    let mut req = [0u8; NIC_HDR_LEN];
    let rid = seq::next();
    write_request(&mut req, OP_RX_PACKET, rid, 0);
    let mut resp = vec![0u8; RESP_CAP];
    let n = mk_ipc_call(nic_port as u64, req.as_ptr(), NIC_HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 {
        return Err(RxError::SendFailed);
    }
    let (op, _, plen) = parse_response(&resp).ok_or(RxError::BadResponse)?;
    if op != OP_RX_PACKET {
        return Err(RxError::BadResponse);
    }
    let body = NIC_HDR_LEN + 4;
    if (plen as usize) < 4 || resp.len() < body {
        return Err(RxError::BadResponse);
    }
    let status = i32::from_le_bytes(resp[NIC_HDR_LEN..body].try_into().unwrap());
    if status != 0 {
        return Err(RxError::Empty);
    }
    if (plen as usize) < 8 || resp.len() < body + 4 {
        return Err(RxError::BadResponse);
    }
    let frame_len = u32::from_le_bytes(resp[body..body + 4].try_into().unwrap()) as usize;
    let frame_start = body + 4;
    if frame_start + frame_len > resp.len() {
        return Err(RxError::BadResponse);
    }
    Ok(resp[frame_start..frame_start + frame_len].to_vec())
}
