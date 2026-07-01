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

use crate::browser::html::entity::push_decoded;

pub fn read_entity(chars: &mut core::iter::Peekable<core::str::CharIndices>, buf: &mut String) {
    let mut name = String::new();
    while let Some(&(_, c)) = chars.peek() {
        if c == ';' || name.len() >= 12 {
            chars.next();
            break;
        }
        if !c.is_ascii_alphanumeric() && c != '#' {
            break;
        }
        name.push(c);
        chars.next();
    }
    if name.is_empty() {
        buf.push('&');
    } else {
        push_decoded(buf, &name);
    }
}
