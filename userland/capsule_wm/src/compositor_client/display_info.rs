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

use super::wire::call_payload;

const OP: u16 = 0x0008;
const BODY_LEN: usize = 16;

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
}

pub fn query(compositor_port: u32, request_id: u32) -> Result<DisplayInfo, &'static str> {
    let mut body = [0u8; BODY_LEN];
    let status = call_payload(compositor_port, OP, request_id, &[], &mut body)?;
    if status != 0 {
        return Err("compositor rejected display_info");
    }
    let width = u32::from_le_bytes(body[0..4].try_into().map_err(|_| "display width missing")?);
    let height = u32::from_le_bytes(body[4..8].try_into().map_err(|_| "display height missing")?);
    if width == 0 || height == 0 {
        return Err("display info invalid");
    }
    Ok(DisplayInfo { width, height })
}
