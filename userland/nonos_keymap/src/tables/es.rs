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

// Spanish (Spain). Letters stay on QWERTY positions with ntilde on the US
// semicolon key; inverted punctuation sits next to the digit row. Dead
// keys (acute, grave, diaeresis) arrive as plain characters.

pub(crate) fn symbol(base: u8, shift: bool) -> u32 {
    let (lo, hi): (u32, u32) = match base {
        b'2' => (b'2' as u32, b'"' as u32),
        b'3' => (b'3' as u32, 0x00B7), // middle dot
        b'6' => (b'6' as u32, b'&' as u32),
        b'7' => (b'7' as u32, b'/' as u32),
        b'8' => (b'8' as u32, b'(' as u32),
        b'9' => (b'9' as u32, b')' as u32),
        b'0' => (b'0' as u32, b'=' as u32),
        b'-' => (b'\'' as u32, b'?' as u32),
        b'=' => (0x00A1, 0x00BF), // inverted exclamation / question
        b'[' => (b'`' as u32, b'^' as u32),
        b']' => (b'+' as u32, b'*' as u32),
        b';' => (0x00F1, 0x00D1),  // ntilde
        b'\'' => (0x00B4, 0x00A8), // acute accent / diaeresis
        b'\\' => (0x00E7, 0x00C7), // c-cedilla
        b'`' => (0x00BA, 0x00AA),  // masculine / feminine ordinal
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
