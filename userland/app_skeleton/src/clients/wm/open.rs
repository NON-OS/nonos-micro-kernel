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

use crate::wire::{call_payload, read_u32, HDR_LEN, NWMP_MAGIC};

const OP: u16 = 0x0002;
const BODY_LEN: usize = 24;
const OPEN_RESP_LEN: usize = 20;

#[derive(Clone, Copy)]
pub struct WindowPlacement {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn window_open(
    port: u32,
    request_id: u32,
    window_id: u32,
    kind: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<WindowPlacement, &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&window_id.to_le_bytes());
    body[4..8].copy_from_slice(&kind.to_le_bytes());
    body[8..12].copy_from_slice(&x.to_le_bytes());
    body[12..16].copy_from_slice(&y.to_le_bytes());
    body[16..20].copy_from_slice(&width.to_le_bytes());
    body[20..24].copy_from_slice(&height.to_le_bytes());
    let mut rx = [0u8; HDR_LEN + OPEN_RESP_LEN];
    let n = call_payload(port, NWMP_MAGIC, OP, request_id, &body, &mut rx)?;
    if n < HDR_LEN + OPEN_RESP_LEN {
        return Ok(WindowPlacement { x, y, width, height });
    }
    Ok(WindowPlacement {
        x: read_u32(&rx, HDR_LEN + 4)?,
        y: read_u32(&rx, HDR_LEN + 8)?,
        width: read_u32(&rx, HDR_LEN + 12)?,
        height: read_u32(&rx, HDR_LEN + 16)?,
    })
}
