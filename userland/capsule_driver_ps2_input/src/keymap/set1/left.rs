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
use super::keycodes::KEYCODE_LCTRL;

pub(super) fn keycode_for(scan: u8) -> Option<u32> {
    let v = match scan {
        0x01 => 0x1B,
        0x02 => b'1' as u32,
        0x03 => b'2' as u32,
        0x04 => b'3' as u32,
        0x05 => b'4' as u32,
        0x06 => b'5' as u32,
        0x07 => b'6' as u32,
        0x08 => b'7' as u32,
        0x09 => b'8' as u32,
        0x0A => b'9' as u32,
        0x0B => b'0' as u32,
        0x0C => b'-' as u32,
        0x0D => b'=' as u32,
        0x0E => 0x08,
        0x0F => b'\t' as u32,
        0x10 => b'q' as u32,
        0x11 => b'w' as u32,
        0x12 => b'e' as u32,
        0x13 => b'r' as u32,
        0x14 => b't' as u32,
        0x15 => b'y' as u32,
        0x16 => b'u' as u32,
        0x17 => b'i' as u32,
        0x18 => b'o' as u32,
        0x19 => b'p' as u32,
        0x1A => b'[' as u32,
        0x1B => b']' as u32,
        0x1C => 0x0D,
        0x1D => KEYCODE_LCTRL,
        _ => return None,
    };
    Some(v)
}
