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

use alloc::string::String;

pub fn read_to_gt(chars: &mut core::iter::Peekable<core::str::CharIndices>) -> String {
    let mut raw = String::new();
    while let Some(&(_, c)) = chars.peek() {
        chars.next();
        if c == '>' {
            break;
        }
        if raw.len() < 8192 {
            raw.push(c);
        }
    }
    raw
}
