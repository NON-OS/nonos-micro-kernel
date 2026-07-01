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

use crate::browser::html::parse::read_to_gt::read_to_gt;
use crate::browser::html::parse::tag_name::tag_name;

const CAP: usize = 1_048_576;

pub fn raw_text(chars: &mut core::iter::Peekable<core::str::CharIndices>, name: &str) -> String {
    let mut s = String::new();
    while let Some((_, c)) = chars.next() {
        if c == '<' && chars.peek().map(|&(_, n)| n) == Some('/') {
            chars.next();
            let raw = read_to_gt(chars);
            if tag_name(&raw) == name {
                break;
            }
            if s.len() < CAP {
                s.push('<');
                s.push('/');
                s.push_str(&raw);
                s.push('>');
            }
        } else if s.len() < CAP {
            s.push(c);
        }
    }
    s
}
