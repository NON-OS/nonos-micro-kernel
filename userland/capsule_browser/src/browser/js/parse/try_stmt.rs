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

use crate::browser::js::ast::Stmt;
use crate::browser::js::token::Tok;

use super::block::block;
use super::parser::Parser;

// `try { .. }` with an optional `catch (e) { .. }` and/or `finally { .. }`. The
// leading `try` keyword has already been consumed.
pub fn parse_try(p: &mut Parser) -> Stmt {
    let body = block(p);
    let mut catch = None;
    if p.eat_kw("catch") {
        let param = if p.eat_punct("(") {
            let name = match p.advance() {
                Tok::Ident(s) => Some(s),
                _ => None,
            };
            p.eat_punct(")");
            name
        } else {
            None
        };
        catch = Some((param, block(p)));
    }
    let finally = if p.eat_kw("finally") { Some(block(p)) } else { None };
    Stmt::Try(body, catch, finally)
}
