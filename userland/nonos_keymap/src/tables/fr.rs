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

// French AZERTY. A/Q and Z/W trade places (M is handled by the resolver:
// it lives at the US semicolon position, and the US m position produces
// comma). Digits need shift; the unshifted row carries the accented
// letters. Dead keys (circumflex, diaeresis) arrive as plain characters,
// composition is not modeled.

pub(crate) fn letter(base: u8) -> u8 {
    match base {
        b'q' => b'a',
        b'a' => b'q',
        b'w' => b'z',
        b'z' => b'w',
        other => other,
    }
}

pub(crate) fn symbol(base: u8, shift: bool) -> u32 {
    let (lo, hi): (u32, u32) = match base {
        b'1' => (b'&' as u32, b'1' as u32),
        b'2' => (0x00E9, b'2' as u32), // e-acute
        b'3' => (b'"' as u32, b'3' as u32),
        b'4' => (b'\'' as u32, b'4' as u32),
        b'5' => (b'(' as u32, b'5' as u32),
        b'6' => (b'-' as u32, b'6' as u32),
        b'7' => (0x00E8, b'7' as u32), // e-grave
        b'8' => (b'_' as u32, b'8' as u32),
        b'9' => (0x00E7, b'9' as u32), // c-cedilla
        b'0' => (0x00E0, b'0' as u32), // a-grave
        b'-' => (b')' as u32, 0x00B0), // degree sign
        b'=' => (b'=' as u32, b'+' as u32),
        b'[' => (b'^' as u32, 0x00A8), // diaeresis
        b']' => (b'$' as u32, 0x00A3), // pound sign
        b'm' => (b',' as u32, b'?' as u32),
        b'\'' => (0x00F9, b'%' as u32), // u-grave
        b'\\' => (b'*' as u32, 0x00B5), // micro sign
        b'`' => (0x00B2, 0x00B2),       // superscript two
        b',' => (b';' as u32, b'.' as u32),
        b'.' => (b':' as u32, b'/' as u32),
        b'/' => (b'!' as u32, 0x00A7), // section sign
        other => return super::us::symbol(other, shift),
    };
    if shift {
        hi
    } else {
        lo
    }
}
