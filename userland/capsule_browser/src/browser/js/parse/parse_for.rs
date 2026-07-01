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

use crate::browser::js::ast::Stmt;

use super::body::body;
use super::expr::expr;
use super::parser::Parser;
use super::var::var_decl;

pub fn parse_for(p: &mut Parser) -> Stmt {
    p.eat_punct("(");
    let init = if p.eat_punct(";") {
        None
    } else if p.is_kw("var") || p.is_kw("let") || p.is_kw("const") {
        p.advance();
        Some(Box::new(var_decl(p)))
    } else {
        let e = expr(p);
        p.eat_punct(";");
        Some(Box::new(Stmt::Expr(e)))
    };
    let cond = if p.is_punct(";") { None } else { Some(expr(p)) };
    p.eat_punct(";");
    let update = if p.is_punct(")") { None } else { Some(expr(p)) };
    p.eat_punct(")");
    Stmt::For(init, cond, update, body(p))
}
