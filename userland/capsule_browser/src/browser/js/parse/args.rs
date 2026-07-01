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

use crate::browser::js::ast::Expr;
use crate::browser::js::token::Tok;

use super::expr::expr;
use super::parser::Parser;

pub fn args(p: &mut Parser) -> Vec<Expr> {
    p.eat_punct("(");
    let mut out: Vec<Expr> = Vec::new();
    while !p.is_punct(")") && !matches!(p.peek(), Tok::Eof) {
        out.push(expr(p));
        if !p.eat_punct(",") {
            break;
        }
        if out.len() >= 1024 {
            break;
        }
    }
    p.eat_punct(")");
    out
}
