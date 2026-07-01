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

use crate::browser::js::ast::Stmt;

use super::block::block;
use super::expr::expr;
use super::func_decl::func_decl;
use super::parse_for::parse_for;
use super::parse_if::parse_if;
use super::parse_return::parse_return;
use super::parse_while::parse_while;
use super::parser::Parser;
use super::var::var_decl;

pub fn statement(p: &mut Parser) -> Stmt {
    p.depth += 1;
    if p.depth > 400 {
        p.depth -= 1;
        return Stmt::Break;
    }
    let s = if p.is_kw("var") || p.is_kw("let") || p.is_kw("const") {
        p.advance();
        var_decl(p)
    } else if p.eat_kw("if") {
        parse_if(p)
    } else if p.eat_kw("while") {
        parse_while(p)
    } else if p.eat_kw("for") {
        parse_for(p)
    } else if p.eat_kw("function") {
        func_decl(p)
    } else if p.eat_kw("return") {
        parse_return(p)
    } else if p.eat_kw("break") {
        p.eat_punct(";");
        Stmt::Break
    } else if p.eat_kw("continue") {
        p.eat_punct(";");
        Stmt::Continue
    } else if p.is_punct("{") {
        Stmt::Block(block(p))
    } else if p.eat_punct(";") {
        Stmt::Block(Vec::new())
    } else {
        let e = expr(p);
        p.eat_punct(";");
        Stmt::Expr(e)
    };
    p.depth -= 1;
    s
}
