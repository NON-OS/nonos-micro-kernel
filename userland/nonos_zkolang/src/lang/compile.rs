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

//! Lowering: the AST to a flat VM program. The language is single-assignment at
//! the source level, but the compiler reuses physical registers: once a temporary
//! subexpression value has been consumed by its parent, its register returns to a
//! free pool for the next temporary. This keeps register pressure at the depth of
//! the expression rather than its size, so real programs like a range proof fit in
//! the sixteen-register file. Register indices stay compile-time constants, which
//! is all the step AIR's register binding needs; reuse is invisible to it. A name
//! resolves to the most recent `let` that bound it, giving lexical shadowing.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::parse::{Ast, Expr, Stmt};
use super::CompileError;
use crate::isa::{Op, REGS};

struct Compiler {
    ops: Vec<Op>,
    syms: Vec<(String, u8)>,
    // The high-water mark of registers ever allocated, and the pool of registers
    // freed from dead temporaries and available for reuse.
    next: u8,
    free: Vec<u8>,
    // The number of public inputs; public inputs take indices `0..n_public` and
    // private (secret) inputs take indices from `n_public` on, so the public
    // inputs are a prefix the AIR binds and the secrets are a hidden suffix.
    n_public: u16,
    next_public: u16,
    next_secret: u16,
    next_output: u16,
}

// A compiled subexpression: the register holding its value, and whether that
// register is a temporary (safe to free once consumed) rather than a live binding.
struct Val {
    reg: u8,
    temp: bool,
}

impl Compiler {
    // Reserve a register, reusing a freed one when the pool is non-empty.
    fn alloc(&mut self) -> Result<u8, CompileError> {
        if let Some(r) = self.free.pop() {
            return Ok(r);
        }
        if self.next as usize >= REGS {
            return Err(CompileError::TooManyRegisters);
        }
        let r = self.next;
        self.next += 1;
        Ok(r)
    }

    // Return a value's register to the pool if it was a temporary.
    fn release(&mut self, v: &Val) {
        if v.temp {
            self.free.push(v.reg);
        }
    }

    // The register a bound name currently resolves to, newest binding first.
    fn lookup(&self, name: &str) -> Option<u8> {
        self.syms.iter().rev().find(|(n, _)| n.as_str() == name).map(|(_, r)| *r)
    }

    fn stmt(&mut self, s: &Stmt) -> Result<(), CompileError> {
        match s {
            Stmt::Let(name, e) => {
                // The result becomes a live binding, so it is not released.
                let v = self.expr(e)?;
                self.syms.push((name.clone(), v.reg));
                Ok(())
            }
            Stmt::Assert(e) => {
                let v = self.expr(e)?;
                self.ops.push(Op::Assert { a: v.reg });
                self.release(&v);
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
                let v = self.expr(e)?;
                let idx = self.next_output;
                self.next_output += 1;
                self.ops.push(Op::Out { a: v.reg, idx });
                self.release(&v);
                Ok(())
            }
        }
    }

    // The shared shape of a two-operand arithmetic node: compile both operands,
    // release any temporaries so the result can reuse their registers, allocate
    // the result, and emit the op.
    fn binary(
        &mut self,
        l: &Expr,
        r: &Expr,
        make: fn(u8, u8, u8) -> Op,
    ) -> Result<Val, CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&a);
        self.release(&b);
        let d = self.alloc()?;
        self.ops.push(make(d, a.reg, b.reg));
        Ok(Val { reg: d, temp: true })
    }

    // Compile an expression, returning the register that holds its value.
    fn expr(&mut self, e: &Expr) -> Result<Val, CompileError> {
        match e {
            Expr::Num(v) => {
                let d = self.alloc()?;
                self.ops.push(Op::Imm { d, v: Fp::from_u64(*v) });
                Ok(Val { reg: d, temp: true })
            }
            Expr::Var(n) => {
                let reg = self.lookup(n).ok_or(CompileError::UnknownVariable)?;
                Ok(Val { reg, temp: false })
            }
            Expr::Add(l, r) => self.binary(l, r, |d, a, b| Op::Add { d, a, b }),
            Expr::Sub(l, r) => self.binary(l, r, |d, a, b| Op::Sub { d, a, b }),
            Expr::Mul(l, r) => self.binary(l, r, |d, a, b| Op::Mul { d, a, b }),
            Expr::Eq(l, r) => self.binary(l, r, |d, a, b| Op::Eq { d, a, b }),
            // Division is sugar with no opcode of its own: a / b is a * b^{-1}. We
            // invert the divisor, then multiply. Because inverting zero has no
            // valid trace, dividing by zero is unprovable rather than a wrong
            // answer, which is the honest behaviour.
            Expr::Div(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                // The divisor is consumed by the inverse; free it so the reciprocal
                // can reuse its register.
                self.release(&b);
                let recip = self.alloc()?;
                self.ops.push(Op::Inv { d: recip, a: b.reg });
                // The dividend and the reciprocal are consumed by the multiply.
                self.release(&a);
                self.free.push(recip);
                let d = self.alloc()?;
                self.ops.push(Op::Mul { d, a: a.reg, b: recip });
                Ok(Val { reg: d, temp: true })
            }
            // Negation is subtraction from zero: -x = 0 - x. We load a zero, then
            // subtract, so no dedicated opcode is needed.
            Expr::Neg(x) => {
                let v = self.expr(x)?;
                let zero = self.alloc()?;
                self.ops.push(Op::Imm { d: zero, v: Fp::ZERO });
                self.release(&v);
                self.free.push(zero);
                let d = self.alloc()?;
                self.ops.push(Op::Sub { d, a: zero, b: v.reg });
                Ok(Val { reg: d, temp: true })
            }
            // Not-equal is the complement of the equality bit: (a != b) = 1 - (a == b).
            // We compute the equality bit, then subtract it from one, which flips a
            // clean zero-or-one bit to its opposite.
            Expr::Ne(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                self.release(&a);
                self.release(&b);
                let bit = self.alloc()?;
                self.ops.push(Op::Eq { d: bit, a: a.reg, b: b.reg });
                let one = self.alloc()?;
                self.ops.push(Op::Imm { d: one, v: Fp::ONE });
                // Both the equality bit and the one are consumed by the subtract.
                self.free.push(bit);
                self.free.push(one);
                let d = self.alloc()?;
                self.ops.push(Op::Sub { d, a: one, b: bit });
                Ok(Val { reg: d, temp: true })
            }
            Expr::Inv(x) => {
                let a = self.expr(x)?;
                self.release(&a);
                let d = self.alloc()?;
                self.ops.push(Op::Inv { d, a: a.reg });
                Ok(Val { reg: d, temp: true })
            }
            Expr::Sel(cond, l, r) => {
                let c = self.expr(cond)?;
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                self.release(&c);
                self.release(&a);
                self.release(&b);
                let d = self.alloc()?;
                self.ops.push(Op::Sel { d, c: c.reg, a: a.reg, b: b.reg });
                Ok(Val { reg: d, temp: true })
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
        free: Vec::new(),
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
