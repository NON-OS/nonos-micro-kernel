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

use crate::browser::js::token::Tok;

pub fn scan_string(cs: &[char], start: usize) -> (Tok, usize) {
    let quote = cs[start];
    let mut i = start + 1;
    let mut s = String::new();
    while i < cs.len() && cs[i] != quote {
        if cs[i] == '\\' && i + 1 < cs.len() {
            i += 1;
            s.push(match cs[i] {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '0' => '\0',
                other => other,
            });
        } else {
            s.push(cs[i]);
        }
        i += 1;
        if s.len() >= 1_048_576 {
            break;
        }
    }
    (Tok::Str(s), (i + 1).min(cs.len()))
}
