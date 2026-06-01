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

use super::super::wire::{call_status, NIRS_MAGIC};

const OP: u16 = 0x0002;

pub fn subscribe(port: u32, request_id: u32, kind_mask: u32) -> Result<(), &'static str> {
    let mut body = [0u8; 8];
    body[0..4].copy_from_slice(&kind_mask.to_le_bytes());
    if call_status(port, NIRS_MAGIC, OP, request_id, &body)? != 0 {
        return Err("input_router rejected subscribe");
    }
    Ok(())
}
