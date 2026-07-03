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
use alloc::string::String;

use crate::browser::js::ast::Expr;
use crate::browser::js::token::Tok;

use super::binary::binary;
use super::parser::Parser;

pub fn expr(p: &mut Parser) -> Expr {
    let left = binary(p, 1);
    if p.is_punct("=")
        || p.is_punct("+=")
        || p.is_punct("-=")
        || p.is_punct("*=")
        || p.is_punct("/=")
    {
        let op = match p.advance() {
            Tok::Punct(s) => s,
            _ => String::from("="),
        };
        let right = expr(p);
        return Expr::Assign(op, Box::new(left), Box::new(right));
    }
    if p.eat_punct("?") {
        let then = expr(p);
        p.eat_punct(":");
        let other = expr(p);
        return Expr::Ternary(Box::new(left), Box::new(then), Box::new(other));
    }
    left
}
