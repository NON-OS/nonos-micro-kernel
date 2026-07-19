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

//! The zkolang source language front-end.
//!
//! A small, total, straight-line language that compiles to the VM instruction
//! set the step AIR proves. A program is a sequence of `let` bindings and
//! `assert` statements over field values:
//!
//! ```text
//!   let a = 3;
//!   let b = 5;
//!   let s = a + b;      // add
//!   let p = s * s;      // multiply
//!   let q = inv(b);     // field inverse
//!   let eqv = s == 8;   // equality yields a 0/1 bit
//!   let pick = sel(eqv, a, b);  // branchless conditional
//!   assert p - 64;      // assert an expression is zero
//! ```
//!
//! The grammar, lowest precedence first:
//!
//! ```text
//!   program := stmt*
//!   stmt    := 'let' ident '=' expr ';' | 'assert' expr ';'
//!   expr    := equality
//!   equality:= sum ('==' sum)?
//!   sum     := product (('+' | '-') product)*
//!   product := primary ('*' primary)*
//!   primary := number | ident | '(' expr ')'
//!            | 'inv' '(' expr ')'
//!            | 'sel' '(' expr ',' expr ',' expr ')'
//! ```
//!
//! Every value is single-assignment: each subexpression takes a fresh register,
//! which is why the compiled trace binds cleanly to the register file. There are
//! no loops or jumps in the surface language yet; a bounded `for` is unrolled by
//! the front-end before it reaches the compiler, so a program's step count is
//! always a static property. `assert e` compiles to the zero-assertion opcode, so
//! it reads as "e must be zero"; write `assert x - y` to require `x == y`.

mod compile;
mod lex;
mod parse;

pub use compile::compile;

/// Anything that can go wrong turning source into a program. The front-end never
/// panics: a malformed program is one of these, with the byte offset when the
/// lexer can point at the offending character.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CompileError {
    /// A character that starts no token, at this byte offset.
    UnexpectedChar { at: usize },
    /// A numeric literal that does not fit in the field's 64-bit representative.
    NumberTooLarge { at: usize },
    /// The token stream ended in the middle of a statement or expression.
    UnexpectedEof,
    /// A token that does not fit the grammar at this point.
    UnexpectedToken,
    /// A reference to a name that was never bound by a `let`.
    UnknownVariable,
    /// The program needs more live values than the register file holds.
    TooManyRegisters,
    /// A `for` loop whose range would unroll to too many iterations.
    LoopTooLarge,
    /// A call to a function that was never defined.
    UnknownFunction,
    /// A call whose argument count does not match the function's parameters.
    ArityMismatch,
    /// Function inlining nested too deep, which a recursive call would cause.
    RecursionTooDeep,
}

use crate::isa::Op;
use alloc::vec::Vec;

/// Compile zkolang source into a VM program ending in `Halt`, ready for the VM to
/// run and the step AIR to prove.
pub fn compile_source(src: &str) -> Result<Vec<Op>, CompileError> {
    let tokens = lex::lex(src)?;
    let ast = parse::parse(&tokens)?;
    compile(&ast)
}
