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
use alloc::vec::Vec;

use crate::browser::js::token::Tok;

use super::parser::Parser;

pub fn params(p: &mut Parser) -> Vec<String> {
    p.eat_punct("(");
    let mut out: Vec<String> = Vec::new();
    while !p.is_punct(")") && !matches!(p.peek(), Tok::Eof) {
        if let Tok::Ident(s) = p.advance() {
            out.push(s);
        }
        if !p.eat_punct(",") {
            break;
        }
        if out.len() >= 256 {
            break;
        }
    }
    p.eat_punct(")");
    out
}
