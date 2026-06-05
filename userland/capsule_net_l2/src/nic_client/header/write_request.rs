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

use super::super::wire::{NIC_HDR_LEN, NIC_MAGIC, NIC_VERSION};

pub fn write_request(out: &mut [u8], op: u16, request_id: u32, payload_len: u32) -> Option<usize> {
    if out.len() < NIC_HDR_LEN {
        return None;
    }
    out[0..4].copy_from_slice(&NIC_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&NIC_VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
    Some(NIC_HDR_LEN + payload_len as usize)
}
