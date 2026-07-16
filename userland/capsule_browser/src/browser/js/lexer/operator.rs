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

use alloc::string::{String, ToString};

use crate::browser::js::token::Tok;

const THREE: [&str; 2] = ["===", "!=="];
const TWO: [&str; 13] =
    ["==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "++", "--", "=>"];

pub fn scan_op(cs: &[char], i: usize) -> (Tok, usize) {
    if i + 3 <= cs.len() {
        let t: String = cs[i..i + 3].iter().collect();
        if THREE.contains(&t.as_str()) {
            return (Tok::Punct(t), i + 3);
        }
    }
    if i + 2 <= cs.len() {
        let t: String = cs[i..i + 2].iter().collect();
        if TWO.contains(&t.as_str()) {
            return (Tok::Punct(t), i + 2);
        }
    }
    (Tok::Punct(cs[i].to_string()), i + 1)
}
