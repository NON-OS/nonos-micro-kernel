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
use alloc::vec;

use crate::browser::js::ast::Expr;
use crate::browser::js::token::Tok;

use super::block::block;
use super::params::params;
use super::parser::Parser;
use super::primary::arrow;

// The `async` keyword has already been consumed. Parse the async function
// expression or arrow that follows; a bare `async` with no callable form is
// treated as an ordinary identifier.
pub(super) fn parse_async(p: &mut Parser) -> Expr {
    if p.eat_kw("function") {
        if matches!(p.peek(), Tok::Ident(_)) {
            p.advance();
        }
        return Expr::Func(params(p), block(p), true);
    }
    if p.is_punct("(") {
        let ps = params(p);
        p.eat_punct("=>");
        return arrow(ps, p, true);
    }
    if matches!(p.peek(), Tok::Ident(_)) && matches!(p.peek2(), Tok::Punct(a) if a == "=>") {
        let name = match p.advance() {
            Tok::Ident(s) => s,
            _ => String::new(),
        };
        p.eat_punct("=>");
        return arrow(vec![name], p, true);
    }
    Expr::Ident("async".to_string())
}
