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

use crate::wire::{call_status, NWMP_MAGIC};

const OP_MOVE: u16 = 0x0004;
const BODY_LEN: usize = 16;

pub fn window_move(
    port: u32,
    request_id: u32,
    window_id: u32,
    x: u32,
    y: u32,
) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&window_id.to_le_bytes());
    body[8..12].copy_from_slice(&x.to_le_bytes());
    body[12..16].copy_from_slice(&y.to_le_bytes());
    let status = call_status(port, NWMP_MAGIC, OP_MOVE, request_id, &body)?;
    if status != 0 {
        return Err("wm rejected window_move");
    }
    Ok(())
}
