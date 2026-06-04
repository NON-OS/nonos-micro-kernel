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

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_ADDRESS_DEVICE, STATUS_LEN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_LEN: usize = 8;

#[derive(Clone, Copy, Debug)]
pub struct AddressedDevice {
    pub slot_id: u8,
    pub port_id: u8,
    pub speed: u8,
    pub max_packet_size: u16,
}

pub fn address_device(
    xhci_port: u32,
    slot: u8,
    port: u8,
) -> Result<AddressedDevice, XhciClientError> {
    let mut resp = [0u8; HDR_AND_STATUS + REPLY_LEN];
    let (status, data_len) = call(xhci_port, OP_ADDRESS_DEVICE, &[slot, port], &mut resp)?;
    if status != 0 || data_len < REPLY_LEN {
        return Err(XhciClientError::BadResponse);
    }
    let o = HDR_AND_STATUS;
    let max_packet = u16::from_le_bytes([resp[o + 4], resp[o + 5]]);
    Ok(AddressedDevice {
        slot_id: resp[o],
        port_id: resp[o + 1],
        speed: resp[o + 2],
        max_packet_size: max_packet,
    })
}
