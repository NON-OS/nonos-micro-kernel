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

use super::parser::Parser;
use super::prec::prec;
use super::unary::unary;

pub fn binary(p: &mut Parser, min: u8) -> Expr {
    let mut left = unary(p);
    while let Tok::Punct(op) = p.peek().clone() {
        let pr = prec(&op);
        if pr == 0 || pr < min {
            break;
        }
        p.advance();
        let right = binary(p, pr + 1);
        left = if op == "&&" || op == "||" {
            Expr::Logical(op, Box::new(left), Box::new(right))
        } else {
            Expr::Binary(op, Box::new(left), Box::new(right))
        };
    }
    left
}
