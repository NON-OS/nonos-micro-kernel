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
use alloc::vec::Vec;

use super::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expr(Expr),
    Var(Vec<(String, Option<Expr>)>),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Vec<Stmt>),
    ForOf(String, Expr, Vec<Stmt>),
    // Named function declaration: name, parameters, body, and the `async` flag.
    Func(String, Vec<String>, Vec<Stmt>, bool),
    Return(Option<Expr>),
    Break,
    Continue,
    Block(Vec<Stmt>),
    // `class Name extends Super { method(params) { body } ... }`. The optional
    // expression is the superclass; each method carries its name, params and body
    // (the constructor is the method named "constructor").
    Class(String, Option<Expr>, Vec<ClassMethod>),
    // `throw expr`.
    Throw(Expr),
    // `try { .. } catch (e) { .. } finally { .. }`: the try body, an optional
    // catch (binding name and body), and an optional finally body.
    Try(Vec<Stmt>, Option<(Option<String>, Vec<Stmt>)>, Option<Vec<Stmt>>),
}

/// One method in a class body.
#[derive(Clone)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub is_async: bool,
}
