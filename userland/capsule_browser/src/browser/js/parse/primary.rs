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
use alloc::vec;
use alloc::vec::Vec;

use crate::browser::js::ast::{Expr, Stmt};
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
        Tok::Tmpl(raw) => super::template::template(&raw),
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
            // A bare identifier followed by `=>` is a single-parameter arrow
            // function: `x => body`.
            _ => {
                if p.is_punct("=>") {
                    p.advance();
                    arrow(vec![s], p)
                } else {
                    Expr::Ident(s)
                }
            }
        },
        Tok::Punct(op) => match op.as_str() {
            "(" => paren_or_arrow(p),
            "[" => array_lit(p),
            "{" => object_lit(p),
            _ => Expr::Undef,
        },
        Tok::Eof => Expr::Undef,
    }
}

// A `(` opens either a parenthesised expression or an arrow parameter list. The
// group is parsed as a comma-separated expression list; if `=>` follows the
// closing `)`, the items (which must be identifiers) become the parameters,
// otherwise it is an ordinary parenthesised expression.
fn paren_or_arrow(p: &mut Parser) -> Expr {
    if p.is_punct(")") {
        p.advance();
        if p.is_punct("=>") {
            p.advance();
            return arrow(Vec::new(), p);
        }
        return Expr::Undef;
    }
    let mut items = vec![expr(p)];
    while p.eat_punct(",") {
        items.push(expr(p));
    }
    p.eat_punct(")");
    if p.is_punct("=>") {
        p.advance();
        let ps = items
            .into_iter()
            .filter_map(|e| match e {
                Expr::Ident(s) => Some(s),
                _ => None,
            })
            .collect();
        return arrow(ps, p);
    }
    items.into_iter().last().unwrap_or(Expr::Undef)
}

// Parse the body of an arrow function: a `{ ... }` block, or a single
// expression that becomes an implicit `return`.
fn arrow(ps: Vec<String>, p: &mut Parser) -> Expr {
    let body = if p.is_punct("{") {
        super::block::block(p)
    } else {
        vec![Stmt::Return(Some(expr(p)))]
    };
    Expr::Func(ps, body)
}
