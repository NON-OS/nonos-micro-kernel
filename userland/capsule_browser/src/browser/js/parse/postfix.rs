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

use alloc::boxed::Box;

use crate::browser::js::ast::Expr;
use crate::browser::js::token::Tok;

use super::args::args;
use super::expr::expr;
use super::parser::Parser;
use super::primary::primary;

pub fn postfix(p: &mut Parser) -> Expr {
    let mut e = primary(p);
    loop {
        if p.eat_punct(".") {
            match p.advance() {
                Tok::Ident(name) => e = Expr::Member(Box::new(e), name),
                _ => break,
            }
        } else if p.is_punct("[") {
            p.advance();
            let idx = expr(p);
            p.eat_punct("]");
            e = Expr::Index(Box::new(e), Box::new(idx));
        } else if p.is_punct("(") {
            e = Expr::Call(Box::new(e), args(p));
        } else {
            break;
        }
    }
    e
}
