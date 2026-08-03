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
// Spanish. Brackets on the two keys right of P, braces on the two keys
// right of L, and @ and # on the number row.

pub(super) fn altgr(base: u8) -> u32 {
    let c = match base {
        b'2' => b'@',
        b'3' => b'#',
        b'[' => b'[',
        b']' => b']',
        b'\'' => b'{',
        b'\\' => b'}',
        b'`' => b'\\',
        b'e' => return 0x20AC, // euro sign
        _ => return 0,
    };
    c as u32
}
