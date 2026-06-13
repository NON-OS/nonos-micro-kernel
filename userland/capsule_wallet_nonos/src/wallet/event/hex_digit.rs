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

pub fn hex_digit(code: u32) -> Option<u8> {
    match code {
        c if (b'0' as u32..=b'9' as u32).contains(&c) => Some((c as u8) - b'0'),
        c if (b'a' as u32..=b'f' as u32).contains(&c) => Some((c as u8) - b'a' + 10),
        c if (b'A' as u32..=b'F' as u32).contains(&c) => Some((c as u8) - b'A' + 10),
        _ => None,
    }
}
