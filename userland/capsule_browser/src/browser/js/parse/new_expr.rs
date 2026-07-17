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
use alloc::vec::Vec;

use crate::browser::js::ast::Expr;
use crate::browser::js::token::Tok;

use super::args::args;
use super::expr::expr;
use super::parser::Parser;
use super::primary::primary;

// Parse `new Target(args)`. The target is a member chain (no call), since the
// first argument list binds to `new`; a trailing call/member is left for the
// caller's postfix loop so `new A().b()` works.
pub fn new_expr(p: &mut Parser) -> Expr {
    p.eat_kw("new");
    let mut callee = if p.is_kw("new") { new_expr(p) } else { primary(p) };
    loop {
        if p.eat_punct(".") {
            match p.advance() {
                Tok::Ident(name) => callee = Expr::Member(Box::new(callee), name),
                _ => break,
            }
        } else if p.is_punct("[") {
            p.advance();
            let idx = expr(p);
            p.eat_punct("]");
            callee = Expr::Index(Box::new(callee), Box::new(idx));
        } else {
            break;
        }
    }
    let a = if p.is_punct("(") { args(p) } else { Vec::new() };
    Expr::New(Box::new(callee), a)
}
