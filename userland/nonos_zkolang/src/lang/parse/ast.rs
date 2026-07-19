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

//! The abstract syntax tree. These are the shapes the parser produces and the
//! compiler walks; the set stays small on purpose, since each node lowers to a
//! short, fixed run of opcodes.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

// An expression node. The tree is what the compiler walks; each variant lowers to
// a small, fixed run of opcodes, which is why the set stays this compact.
#[derive(Clone, Debug)]
pub enum Expr {
    // A literal and a variable reference, the leaves of every tree.
    Num(u64),
    Var(String),
    // Field arithmetic. `Div` is sugar for a multiply by an inverse, and `Neg` for
    // a subtraction from zero, so neither needs its own opcode.
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    // Comparisons that yield a zero or one bit. `Ne` is the complement of `Eq`,
    // lowered as one minus the equality bit.
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    // The field inverse and the branchless select, written as calls.
    Inv(Box<Expr>),
    Sel(Box<Expr>, Box<Expr>, Box<Expr>),
    // A conditional expression, sugar for `sel`: both arms are evaluated and one
    // is chosen by the boolean condition. Order is (cond, then, else).
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    // A call to a user function, inlined at compile time. The name and its
    // argument expressions.
    Call(String, Vec<Expr>),
}

// A statement node.
#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Expr),
    Assert(Expr),
    // Bind a name to the next public input.
    Input(String),
    // Bind a name to the next private input, a witness not in the public statement.
    Secret(String),
    // Expose an expression as the next public output.
    Output(Expr),
    // A bounded loop over `[lo, hi)`, unrolled by the compiler. The loop variable
    // is a compile-time constant in the body, so the body's shape never depends on
    // a runtime value.
    For { var: String, lo: u64, hi: u64, body: Vec<Stmt> },
}

/// A parsed program.
#[derive(Clone, Debug)]
pub struct Ast {
    pub fns: Vec<FnDef>,
    pub stmts: Vec<Stmt>,
}

/// A function definition. Functions are compile-time inlined, so the body is a
/// single expression and the parameters are substituted at each call site; there
/// is no call stack and no recursion.
#[derive(Clone, Debug)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}
