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

pub fn scan_number(cs: &[char], start: usize) -> (Tok, usize) {
    let mut i = start;
    while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
        i += 1;
    }
    if i < cs.len() && (cs[i] == 'e' || cs[i] == 'E') {
        i += 1;
        if i < cs.len() && (cs[i] == '+' || cs[i] == '-') {
            i += 1;
        }
        while i < cs.len() && cs[i].is_ascii_digit() {
            i += 1;
        }
    }
    let s: String = cs[start..i].iter().collect();
    (Tok::Num(s.parse::<f64>().unwrap_or(0.0)), i)
}
