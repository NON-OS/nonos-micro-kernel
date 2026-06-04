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

use nonos_libc::SURFACE_FORMAT_ARGB8888;

use crate::wire::{call_payload, read_u32, HDR_LEN, NCMP_MAGIC};

const OP: u16 = 0x0008;
const BODY_LEN: usize = 16;

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
}

pub fn display_info(port: u32, request_id: u32) -> Result<DisplayInfo, &'static str> {
    let mut rx = [0u8; HDR_LEN + 4 + BODY_LEN];
    let total = call_payload(port, NCMP_MAGIC, OP, request_id, &[], &mut rx)?;
    if total < HDR_LEN + 4 + BODY_LEN {
        return Err("display_info truncated");
    }
    let base = HDR_LEN + 4;
    let width = read_u32(&rx, base)?;
    let height = read_u32(&rx, base + 4)?;
    let stride = read_u32(&rx, base + 8)?;
    let format = read_u32(&rx, base + 12)?;
    if width == 0 || height == 0 || stride == 0 || format != SURFACE_FORMAT_ARGB8888 {
        return Err("display_info invalid");
    }
    Ok(DisplayInfo { width, height })
}
