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

use super::super::capability::gate_call;
use super::super::error::DriverXhciError;
use super::super::protocol::{encode_request, OP_ADDRESS_DEVICE};
use super::seq::next_request_id;
use super::transport::round_trip;

const REPLY_BYTES: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct AddressedDevice {
    pub slot_id: u8,
    pub port_id: u8,
    pub speed: u8,
    pub max_packet_size: u16,
}

pub fn address_device(slot_id: u8, port_id: u8) -> Result<AddressedDevice, DriverXhciError> {
    if slot_id == 0 || port_id == 0 {
        return Err(DriverXhciError::InvalidArgument);
    }
    let _caller = gate_call()?;
    let request_id = next_request_id();
    let frame = encode_request(OP_ADDRESS_DEVICE, 0, request_id, &[slot_id, port_id]);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(DriverXhciError::DeviceFailure);
    }
    decode_reply(&resp.body)
}

fn decode_reply(body: &[u8]) -> Result<AddressedDevice, DriverXhciError> {
    if body.len() < REPLY_BYTES {
        return Err(DriverXhciError::ShortReply);
    }
    let max_packet_size = u16::from_le_bytes([body[4], body[5]]);
    Ok(AddressedDevice { slot_id: body[0], port_id: body[1], speed: body[2], max_packet_size })
}
