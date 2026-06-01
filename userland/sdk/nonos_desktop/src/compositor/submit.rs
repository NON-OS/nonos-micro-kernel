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

use super::super::wire::{call_status, NCMP_MAGIC};

const OP: u16 = 0x0002;

pub fn scene_submit(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    z: u32,
) -> Result<(), &'static str> {
    let mut body = [0u8; 32];
    body[0..8].copy_from_slice(&surface_handle.to_le_bytes());
    body[8..12].copy_from_slice(&x.to_le_bytes());
    body[12..16].copy_from_slice(&y.to_le_bytes());
    body[16..20].copy_from_slice(&width.to_le_bytes());
    body[20..24].copy_from_slice(&height.to_le_bytes());
    body[24..28].copy_from_slice(&z.to_le_bytes());
    if call_status(port, NCMP_MAGIC, OP, request_id, &body)? != 0 {
        return Err("compositor rejected scene_submit");
    }
    Ok(())
}
