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

// Italian. QWERTY letter positions with the accented vowels on the keys
// right of P and the colon/quote positions, matching a physical IT board.

pub(crate) fn symbol(base: u8, shift: bool) -> u32 {
    let (lo, hi): (u32, u32) = match base {
        b'2' => (b'2' as u32, b'"' as u32),
        b'3' => (b'3' as u32, 0x00A3), // pound sign
        b'6' => (b'6' as u32, b'&' as u32),
        b'7' => (b'7' as u32, b'/' as u32),
        b'8' => (b'8' as u32, b'(' as u32),
        b'9' => (b'9' as u32, b')' as u32),
        b'0' => (b'0' as u32, b'=' as u32),
        b'-' => (b'\'' as u32, b'?' as u32),
        b'=' => (0x00EC, b'^' as u32), // i-grave
        b'[' => (0x00E8, 0x00E9),      // e-grave / e-acute
        b']' => (b'+' as u32, b'*' as u32),
        b';' => (0x00F2, 0x00E7),  // o-grave / c-cedilla
        b'\'' => (0x00E0, 0x00B0), // a-grave / degree
        b'\\' => (0x00F9, 0x00A7), // u-grave / section
        b'`' => (b'\\' as u32, b'|' as u32),
        b',' => (b',' as u32, b';' as u32),
        b'.' => (b'.' as u32, b':' as u32),
        b'/' => (b'-' as u32, b'_' as u32),
        other => return super::us::symbol(other, shift),
    };
    if shift {
        hi
    } else {
        lo
    }
}
