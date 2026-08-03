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
// German. Brackets and braces sit across the 7 to 0 row, @ on Q, and the
// backslash and tilde on the two keys right of the zero.

pub(super) fn altgr(base: u8) -> u32 {
    let c = match base {
        b'7' => b'{',
        b'8' => b'[',
        b'9' => b']',
        b'0' => b'}',
        b'-' => b'\\',
        b']' => b'~',
        b'q' => b'@',
        b'e' => return 0x20AC, // euro sign
        b'm' => return 0x00B5, // micro sign
        _ => return 0,
    };
    c as u32
}
