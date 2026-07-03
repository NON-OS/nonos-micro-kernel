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

pub fn parse_hex(h: &str) -> Option<u32> {
    let h = h.trim();
    if !h.is_ascii() {
        return None;
    }
    let b = h.as_bytes();
    // #rgba and #rrggbbaa carry an alpha we drop; colors stay opaque.
    let full: [u8; 6] = match h.len() {
        3 | 4 => [b[0], b[0], b[1], b[1], b[2], b[2]],
        6 | 8 => [b[0], b[1], b[2], b[3], b[4], b[5]],
        _ => return None,
    };
    let s = core::str::from_utf8(&full).ok()?;
    let r = u8::from_str_radix(&s[0..2], 16).ok()? as u32;
    let g = u8::from_str_radix(&s[2..4], 16).ok()? as u32;
    let b = u8::from_str_radix(&s[4..6], 16).ok()? as u32;
    Some(0xFF00_0000 | (r << 16) | (g << 8) | b)
}
