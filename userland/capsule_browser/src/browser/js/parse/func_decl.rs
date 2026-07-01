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

use crate::browser::js::ast::Stmt;
use crate::browser::js::token::Tok;

use super::block::block;
use super::params::params;
use super::parser::Parser;

pub fn func_decl(p: &mut Parser) -> Stmt {
    let name = match p.advance() {
        Tok::Ident(s) => s,
        _ => String::new(),
    };
    let ps = params(p);
    Stmt::Func(name, ps, block(p))
}
