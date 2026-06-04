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
use super::keycodes::{KEYCODE_LALT, KEYCODE_LSHIFT, KEYCODE_RSHIFT};

pub(super) fn keycode_for(scan: u8) -> Option<u32> {
    let v = match scan {
        0x1E => b'a' as u32,
        0x1F => b's' as u32,
        0x20 => b'd' as u32,
        0x21 => b'f' as u32,
        0x22 => b'g' as u32,
        0x23 => b'h' as u32,
        0x24 => b'j' as u32,
        0x25 => b'k' as u32,
        0x26 => b'l' as u32,
        0x27 => b';' as u32,
        0x28 => b'\'' as u32,
        0x29 => b'`' as u32,
        0x2A => KEYCODE_LSHIFT,
        0x2B => b'\\' as u32,
        0x2C => b'z' as u32,
        0x2D => b'x' as u32,
        0x2E => b'c' as u32,
        0x2F => b'v' as u32,
        0x30 => b'b' as u32,
        0x31 => b'n' as u32,
        0x32 => b'm' as u32,
        0x33 => b',' as u32,
        0x34 => b'.' as u32,
        0x35 => b'/' as u32,
        0x36 => KEYCODE_RSHIFT,
        0x37 => b'*' as u32,
        0x38 => KEYCODE_LALT,
        0x39 => b' ' as u32,
        _ => return None,
    };
    Some(v)
}
