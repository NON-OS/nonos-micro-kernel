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
use alloc::vec::Vec;

use crate::browser::js::ast::{ClassMethod, Stmt};
use crate::browser::js::token::Tok;

use super::block::block;
use super::params::params;
use super::parser::Parser;
use super::unary::unary;

// Parse `class Name [extends Super] { method(params) { body } ... }`. Method
// modifiers (static/get/set/async) are accepted and ignored; the method named
// "constructor" is the initialiser.
pub fn class_decl(p: &mut Parser) -> Stmt {
    let name = match p.advance() {
        Tok::Ident(s) => s,
        _ => String::new(),
    };
    let sup = if p.eat_kw("extends") { Some(unary(p)) } else { None };
    p.eat_punct("{");
    let mut methods: Vec<ClassMethod> = Vec::new();
    while !p.is_punct("}") && !matches!(p.peek(), Tok::Eof) {
        if p.eat_punct(";") {
            continue;
        }
        let mut mname = match p.advance() {
            Tok::Ident(s) => s,
            _ => break,
        };
        let mut is_async = false;
        if is_modifier(&mname) && matches!(p.peek(), Tok::Ident(_)) {
            is_async = mname == "async";
            mname = match p.advance() {
                Tok::Ident(s) => s,
                _ => break,
            };
        }
        let ps = params(p);
        methods.push(ClassMethod { name: mname, params: ps, body: block(p), is_async });
        if methods.len() >= 512 {
            break;
        }
    }
    p.eat_punct("}");
    Stmt::Class(name, sup, methods)
}

fn is_modifier(s: &str) -> bool {
    matches!(s, "static" | "get" | "set" | "async")
}
