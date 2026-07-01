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
use crate::browser::js::token::Tok;

use super::parser::Parser;
use super::statement::statement;

pub fn program(p: &mut Parser) -> Vec<Stmt> {
    let mut out: Vec<Stmt> = Vec::new();
    while !matches!(p.peek(), Tok::Eof) && out.len() < 100_000 {
        let before = p.pos;
        out.push(statement(p));
        if p.pos == before {
            p.advance();
        }
    }
    out
}
