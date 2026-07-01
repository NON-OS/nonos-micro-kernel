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

use crate::browser::js::ast::Expr;
use crate::browser::js::token::Tok;

use super::array::array_lit;
use super::expr::expr;
use super::object::object_lit;
use super::params::params;
use super::parser::Parser;

pub fn primary(p: &mut Parser) -> Expr {
    match p.advance() {
        Tok::Num(n) => Expr::Num(n),
        Tok::Str(s) => Expr::Str(s),
        Tok::Ident(s) => match s.as_str() {
            "true" => Expr::Bool(true),
            "false" => Expr::Bool(false),
            "null" => Expr::Null,
            "undefined" => Expr::Undef,
            "function" => {
                if matches!(p.peek(), Tok::Ident(_)) {
                    p.advance();
                }
                Expr::Func(params(p), super::block::block(p))
            }
            _ => Expr::Ident(s),
        },
        Tok::Punct(op) => match op.as_str() {
            "(" => {
                let e = expr(p);
                p.eat_punct(")");
                e
            }
            "[" => array_lit(p),
            "{" => object_lit(p),
            _ => Expr::Undef,
        },
        Tok::Eof => Expr::Undef,
    }
}
