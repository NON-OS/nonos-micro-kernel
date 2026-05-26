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

//! Generic xHCI IPC call: builds 20-byte NXHC header + body,
//! calls `mk_ipc_call`, parses echoed header + status, returns
//! `(status: i32, payload_len: usize)` on success.

use nonos_libc::mk_ipc_call;

use super::seq;
use super::wire::{parse_response, write_request, HDR_LEN, STATUS_LEN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XhciClientError {
    SendFailed,
    BadResponse,
    BufferTooSmall,
}

pub fn call(
    port: u32,
    op: u16,
    body: &[u8],
    resp: &mut [u8],
) -> Result<(i32, usize), XhciClientError> {
    let total_req = HDR_LEN + body.len();
    let mut req = [0u8; HDR_LEN + 64];
    if total_req > req.len() {
        return Err(XhciClientError::BufferTooSmall);
    }
    let rid = seq::next();
    write_request(&mut req, op, rid, body.len() as u32);
    req[HDR_LEN..total_req].copy_from_slice(body);
    let n = mk_ipc_call(
        port as u64,
        req.as_ptr(),
        total_req,
        resp.as_mut_ptr(),
        resp.len(),
    );
    if n < 0 {
        return Err(XhciClientError::SendFailed);
    }
    let (_op, _rid_back, plen) =
        parse_response(resp).ok_or(XhciClientError::BadResponse)?;
    if resp.len() < HDR_LEN + STATUS_LEN {
        return Err(XhciClientError::BadResponse);
    }
    let status = i32::from_le_bytes(
        resp[HDR_LEN..HDR_LEN + STATUS_LEN].try_into().unwrap(),
    );
    let data_len = plen.saturating_sub(STATUS_LEN as u32) as usize;
    Ok((status, data_len))
}
