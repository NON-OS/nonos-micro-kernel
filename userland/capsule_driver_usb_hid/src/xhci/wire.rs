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

//! NXHC wire constants and header codec. Mirrors
//! `capsule_driver_xhci/src/protocol/{header,encode,decode}.rs`.
//! The reply layout is: 20-byte echoed header, then `status: i32`
//! (4 bytes) at offset 20, then payload data bytes.

pub const MAGIC: u32 = 0x4E58_4843;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;

pub const OP_PORT_STATUS: u16 = 0x0003;
pub const OP_ENABLE_SLOT: u16 = 0x0004;
pub const OP_ADDRESS_DEVICE: u16 = 0x0006;
pub const OP_GET_CONFIG_DESCRIPTOR: u16 = 0x0008;
pub const OP_ALLOC_TRANSFER_RING: u16 = 0x0009;
pub const OP_CONTROL_TRANSFER: u16 = 0x000B;
pub const OP_INTERRUPT_IN: u16 = 0x000E;

pub const E_AGAIN: i32 = -11;

pub fn write_request(out: &mut [u8], op: u16, request_id: u32, payload_len: u32) {
    debug_assert!(out.len() >= HDR_LEN);
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
}

pub fn parse_response(buf: &[u8]) -> Option<(u16, u32, u32)> {
    if buf.len() < HDR_LEN {
        return None;
    }
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    if magic != MAGIC || version != VERSION {
        return None;
    }
    let op = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let request_id = u32::from_le_bytes(buf[12..16].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[16..20].try_into().ok()?);
    Some((op, request_id, payload_len))
}
