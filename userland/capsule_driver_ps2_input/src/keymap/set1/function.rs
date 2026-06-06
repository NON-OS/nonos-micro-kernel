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
use super::keycodes::{
    KEYCODE_CAPS, KEYCODE_F1, KEYCODE_F10, KEYCODE_F11, KEYCODE_F12, KEYCODE_F2, KEYCODE_F3,
    KEYCODE_F4, KEYCODE_F5, KEYCODE_F6, KEYCODE_F7, KEYCODE_F8, KEYCODE_F9, KEYCODE_NUMLK,
    KEYCODE_SCROLL,
};

pub(super) fn keycode_for(scan: u8) -> Option<u32> {
    let v = match scan {
        0x3A => KEYCODE_CAPS,
        0x3B => KEYCODE_F1,
        0x3C => KEYCODE_F2,
        0x3D => KEYCODE_F3,
        0x3E => KEYCODE_F4,
        0x3F => KEYCODE_F5,
        0x40 => KEYCODE_F6,
        0x41 => KEYCODE_F7,
        0x42 => KEYCODE_F8,
        0x43 => KEYCODE_F9,
        0x44 => KEYCODE_F10,
        0x45 => KEYCODE_NUMLK,
        0x46 => KEYCODE_SCROLL,
        0x47 => b'7' as u32,
        0x48 => b'8' as u32,
        0x49 => b'9' as u32,
        0x4A => b'-' as u32,
        0x4B => b'4' as u32,
        0x4C => b'5' as u32,
        0x4D => b'6' as u32,
        0x4E => b'+' as u32,
        0x4F => b'1' as u32,
        0x50 => b'2' as u32,
        0x51 => b'3' as u32,
        0x52 => b'0' as u32,
        0x53 => b'.' as u32,
        0x57 => KEYCODE_F11,
        0x58 => KEYCODE_F12,
        _ => return None,
    };
    Some(v)
}
