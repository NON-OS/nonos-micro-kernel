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

// German QWERTZ (T1). Y and Z trade places; umlauts sit on the US
// bracket/semicolon/quote positions; the symbol row follows DIN 2137.
// Shifted umlauts are produced here (the caps path only uppercases
// ASCII), so shift-ü gives Ü; caps-lock plus umlaut stays lowercase,
// a known simplification.

pub(crate) fn letter(base: u8) -> u8 {
    match base {
        b'y' => b'z',
        b'z' => b'y',
        other => other,
    }
}

pub(crate) fn symbol(base: u8, shift: bool) -> u32 {
    let (lo, hi): (u32, u32) = match base {
        b'2' => (b'2' as u32, b'"' as u32),
        b'3' => (b'3' as u32, 0x00A7), // section sign
        b'6' => (b'6' as u32, b'&' as u32),
        b'7' => (b'7' as u32, b'/' as u32),
        b'8' => (b'8' as u32, b'(' as u32),
        b'9' => (b'9' as u32, b')' as u32),
        b'0' => (b'0' as u32, b'=' as u32),
        b'-' => (0x00DF, b'?' as u32), // eszett
        b'=' => (0x00B4, b'`' as u32), // acute accent
        b'[' => (0x00FC, 0x00DC),      // u-umlaut
        b']' => (b'+' as u32, b'*' as u32),
        b';' => (0x00F6, 0x00D6),  // o-umlaut
        b'\'' => (0x00E4, 0x00C4), // a-umlaut
        b'\\' => (b'#' as u32, b'\'' as u32),
        b'`' => (b'^' as u32, 0x00B0), // degree sign
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
