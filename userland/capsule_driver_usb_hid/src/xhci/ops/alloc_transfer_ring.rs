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

//! ALLOC_TRANSFER_RING call. Request body (6 bytes):
//! `[0]=slot,[1]=ep_address,[2..4]=max_packet_le,[4]=interval,[5]=resv`.
//! Reply payload (after status, 4 bytes): `[0]=dci,[1..4]=resv`.
//! Mirrors `xhci/server/handlers/alloc_transfer_ring.rs`.

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_ALLOC_TRANSFER_RING, STATUS_LEN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_LEN: usize = 4;

pub fn alloc_transfer_ring(
    xhci_port: u32,
    slot: u8,
    ep_address: u8,
    max_packet: u16,
    interval: u8,
) -> Result<u8, XhciClientError> {
    let mut body = [0u8; 6];
    body[0] = slot;
    body[1] = ep_address;
    body[2..4].copy_from_slice(&max_packet.to_le_bytes());
    body[4] = interval;
    let mut resp = [0u8; HDR_AND_STATUS + REPLY_LEN];
    let (status, data_len) = call(xhci_port, OP_ALLOC_TRANSFER_RING, &body, &mut resp)?;
    if status != 0 || data_len < REPLY_LEN {
        return Err(XhciClientError::BadResponse);
    }
    Ok(resp[HDR_AND_STATUS])
}
