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

// UK (BS 4822): US positions except quote/at swap, pound on 3, and the
// hash/tilde key beside Enter (the US backslash position on ANSI boards).

pub(crate) fn symbol(base: u8, shift: bool) -> u32 {
    let (lo, hi): (u32, u32) = match base {
        b'2' => (b'2' as u32, b'"' as u32),
        b'3' => (b'3' as u32, 0x00A3), // pound sign
        b'\'' => (b'\'' as u32, b'@' as u32),
        b'\\' => (b'#' as u32, b'~' as u32),
        b'`' => (b'`' as u32, 0x00AC), // not sign
        other => return super::us::symbol(other, shift),
    };
    if shift {
        hi
    } else {
        lo
    }
}
