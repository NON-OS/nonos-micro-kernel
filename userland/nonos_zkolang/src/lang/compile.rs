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

//! Lowering: the AST to a flat VM program. Every subexpression takes a fresh
//! register, so the compiled program is single-assignment and its data flow is
//! exactly the wiring the step AIR binds. A name resolves to the most recent
//! `let` that bound it, which gives ordinary lexical shadowing.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::parse::{Ast, Expr, Stmt};
use super::CompileError;
use crate::isa::{Op, REGS};

struct Compiler {
    ops: Vec<Op>,
    syms: Vec<(String, u8)>,
    next: u8,
    // The number of public inputs; public inputs take indices `0..n_public` and
    // private (secret) inputs take indices from `n_public` on, so the public
    // inputs are a prefix the AIR binds and the secrets are a hidden suffix.
    n_public: u16,
    next_public: u16,
    next_secret: u16,
    next_output: u16,
}

impl Compiler {
    // Reserve the next free register.
    fn alloc(&mut self) -> Result<u8, CompileError> {
        if self.next as usize >= REGS {
            return Err(CompileError::TooManyRegisters);
        }
        let r = self.next;
        self.next += 1;
        Ok(r)
    }

    // The register a bound name currently resolves to, newest binding first.
    fn lookup(&self, name: &str) -> Option<u8> {
        self.syms.iter().rev().find(|(n, _)| n.as_str() == name).map(|(_, r)| *r)
    }

    fn stmt(&mut self, s: &Stmt) -> Result<(), CompileError> {
        match s {
            Stmt::Let(name, e) => {
                let r = self.expr(e)?;
                self.syms.push((name.clone(), r));
                Ok(())
            }
            Stmt::Assert(e) => {
                let r = self.expr(e)?;
                self.ops.push(Op::Assert { a: r });
                Ok(())
            }
            Stmt::Input(name) => {
                let d = self.alloc()?;
                let idx = self.next_public;
                self.next_public += 1;
                self.ops.push(Op::Inp { d, idx });
                self.syms.push((name.clone(), d));
                Ok(())
            }
            Stmt::Secret(name) => {
                let d = self.alloc()?;
                let idx = self.n_public + self.next_secret;
                self.next_secret += 1;
                self.ops.push(Op::Inp { d, idx });
                self.syms.push((name.clone(), d));
                Ok(())
            }
            Stmt::Output(e) => {
                let r = self.expr(e)?;
                let idx = self.next_output;
                self.next_output += 1;
                self.ops.push(Op::Out { a: r, idx });
                Ok(())
            }
        }
    }

    // Compile an expression, returning the register that holds its value.
    fn expr(&mut self, e: &Expr) -> Result<u8, CompileError> {
        match e {
            Expr::Num(v) => {
                let d = self.alloc()?;
                self.ops.push(Op::Imm { d, v: Fp::from_u64(*v) });
                Ok(d)
            }
            Expr::Var(n) => self.lookup(n).ok_or(CompileError::UnknownVariable),
            Expr::Add(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                let d = self.alloc()?;
                self.ops.push(Op::Add { d, a, b });
                Ok(d)
            }
            Expr::Sub(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                let d = self.alloc()?;
                self.ops.push(Op::Sub { d, a, b });
                Ok(d)
            }
            Expr::Mul(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                let d = self.alloc()?;
                self.ops.push(Op::Mul { d, a, b });
                Ok(d)
            }
            Expr::Eq(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                let d = self.alloc()?;
                self.ops.push(Op::Eq { d, a, b });
                Ok(d)
            }
            Expr::Inv(x) => {
                let a = self.expr(x)?;
                let d = self.alloc()?;
                self.ops.push(Op::Inv { d, a });
                Ok(d)
            }
            Expr::Sel(cond, l, r) => {
                let c = self.expr(cond)?;
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                let d = self.alloc()?;
                self.ops.push(Op::Sel { d, c, a, b });
                Ok(d)
            }
        }
    }
}

/// Lower an AST into a VM program ending in `Halt`.
pub fn compile(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    // Count the public inputs first, so secret inputs can be indexed after them.
    let n_public = ast.stmts.iter().filter(|s| matches!(s, Stmt::Input(_))).count() as u16;
    let mut c = Compiler {
        ops: Vec::new(),
        syms: Vec::new(),
        next: 0,
        n_public,
        next_public: 0,
        next_secret: 0,
        next_output: 0,
    };
    for s in &ast.stmts {
        c.stmt(s)?;
    }
    c.ops.push(Op::Halt);
    Ok(c.ops)
}
