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

//! GET_CONFIG_DESCRIPTOR call. Request body (4 bytes): `[0]=slot,
//! [1]=resv,[2..4]=length_le`. Reply payload (after status): 4-byte
//! prefix `[0..2]=actual_len_le,[2..4]=resv`, then descriptor bytes.
//! Mirrors `xhci/server/handlers/config_descriptor.rs`.

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_GET_CONFIG_DESCRIPTOR, STATUS_LEN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_PREFIX: usize = 4;
pub const MAX_DESCRIPTOR_LEN: usize = 512;
const RESP_BUF: usize = HDR_AND_STATUS + REPLY_PREFIX + MAX_DESCRIPTOR_LEN;

pub fn get_config_descriptor(
    xhci_port: u32,
    slot: u8,
    length: u16,
    out: &mut [u8],
) -> Result<usize, XhciClientError> {
    let len = length.to_le_bytes();
    let body = [slot, 0u8, len[0], len[1]];
    let mut resp = [0u8; RESP_BUF];
    let (status, data_len) = call(xhci_port, OP_GET_CONFIG_DESCRIPTOR, &body, &mut resp)?;
    if status != 0 || data_len < REPLY_PREFIX {
        return Err(XhciClientError::BadResponse);
    }
    let o = HDR_AND_STATUS;
    let actual = u16::from_le_bytes([resp[o], resp[o + 1]]) as usize;
    if data_len < REPLY_PREFIX + actual || out.len() < actual {
        return Err(XhciClientError::BadResponse);
    }
    out[..actual].copy_from_slice(&resp[o + REPLY_PREFIX..o + REPLY_PREFIX + actual]);
    Ok(actual)
}
