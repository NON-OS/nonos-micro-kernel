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

// A backtick template, raw: interpolations stay verbatim (brace-balanced)
// for the parser to split; simple escapes resolve here.
pub fn scan_template(cs: &[char], start: usize) -> (Tok, usize) {
    let mut i = start + 1;
    let mut s = String::new();
    while i < cs.len() && cs[i] != '`' {
        if cs[i] == '\\' && i + 1 < cs.len() {
            i += 1;
            s.push(match cs[i] {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                other => other,
            });
            i += 1;
            continue;
        }
        if cs[i] == '$' && i + 1 < cs.len() && cs[i + 1] == '{' {
            s.push('$');
            s.push('{');
            i += 2;
            let mut depth = 1u32;
            while i < cs.len() && depth > 0 {
                match cs[i] {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
                if depth > 0 {
                    s.push(cs[i]);
                }
                i += 1;
            }
            s.push('}');
            continue;
        }
        s.push(cs[i]);
        i += 1;
        if s.len() >= 1_048_576 {
            break;
        }
    }
    (Tok::Tmpl(s), (i + 1).min(cs.len()))
}
