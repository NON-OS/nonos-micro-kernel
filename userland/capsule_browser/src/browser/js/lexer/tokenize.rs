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

use alloc::vec::Vec;

use crate::browser::js::token::Tok;

use super::ident::scan_ident;
use super::number::scan_number;
use super::operator::scan_op;
use super::string::scan_string;
use super::template::scan_template;

pub fn tokenize(src: &str) -> Vec<Tok> {
    let cs: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out: Vec<Tok> = Vec::new();
    while i < cs.len() && out.len() < 200_000 {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '/' && i + 1 < cs.len() && cs[i + 1] == '/' {
            while i < cs.len() && cs[i] != '\n' {
                i += 1;
            }
        } else if c == '/' && i + 1 < cs.len() && cs[i + 1] == '*' {
            i += 2;
            while i + 1 < cs.len() && !(cs[i] == '*' && cs[i + 1] == '/') {
                i += 1;
            }
            i = (i + 2).min(cs.len());
        } else if c.is_ascii_digit() || (c == '.' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit())
        {
            let (t, n) = scan_number(&cs, i);
            out.push(t);
            i = n;
        } else if c == '`' {
            let (t, n) = scan_template(&cs, i);
            out.push(t);
            i = n;
        } else if c == '"' || c == '\'' {
            let (t, n) = scan_string(&cs, i);
            out.push(t);
            i = n;
        } else if c.is_alphabetic() || c == '_' || c == '$' {
            let (t, n) = scan_ident(&cs, i);
            out.push(t);
            i = n;
        } else {
            let (t, n) = scan_op(&cs, i);
            out.push(t);
            i = n;
        }
    }
    out.push(Tok::Eof);
    out
}
