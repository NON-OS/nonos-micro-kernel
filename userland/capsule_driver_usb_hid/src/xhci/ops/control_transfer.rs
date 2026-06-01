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

//! CONTROL_TRANSFER call. Request body (10 bytes):
//! `[0]=slot,[1]=resv,[2]=bmRequestType,[3]=bRequest,
//!  [4..6]=wValue_le,[6..8]=wIndex_le,[8..10]=wLength_le`.
//! Reply payload (after status): `[0..2]=actual_len_le`, then data.
//! Mirrors `xhci/server/handlers/control_transfer.rs`.

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_CONTROL_TRANSFER, STATUS_LEN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_PREFIX: usize = 2;
const MAX_DATA: usize = 512;
const RESP_BUF: usize = HDR_AND_STATUS + REPLY_PREFIX + MAX_DATA;

pub fn control_transfer(
    xhci_port: u32,
    slot: u8,
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
    data_out: &mut [u8],
) -> Result<usize, XhciClientError> {
    let mut body = [0u8; 10];
    body[0] = slot;
    body[2] = bm_request_type;
    body[3] = b_request;
    body[4..6].copy_from_slice(&w_value.to_le_bytes());
    body[6..8].copy_from_slice(&w_index.to_le_bytes());
    body[8..10].copy_from_slice(&w_length.to_le_bytes());
    let mut resp = [0u8; RESP_BUF];
    let (status, data_len) = call(xhci_port, OP_CONTROL_TRANSFER, &body, &mut resp)?;
    if status != 0 || data_len < REPLY_PREFIX {
        return Err(XhciClientError::BadResponse);
    }
    let o = HDR_AND_STATUS;
    let actual = u16::from_le_bytes([resp[o], resp[o + 1]]) as usize;
    if data_len < REPLY_PREFIX + actual || data_out.len() < actual {
        return Err(XhciClientError::BadResponse);
    }
    data_out[..actual].copy_from_slice(&resp[o + REPLY_PREFIX..o + REPLY_PREFIX + actual]);
    Ok(actual)
}
