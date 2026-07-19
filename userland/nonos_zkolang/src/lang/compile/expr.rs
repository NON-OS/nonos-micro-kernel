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

//! Expression lowering: each node becomes a small run of opcodes. Division,
//! negation, and not-equal are sugar over the existing opcodes; the conditional is
//! the branchless select; and a function call is inlined hygienically, its body
//! compiled in place with the parameters bound to the argument registers.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::super::parse::Expr;
use super::super::CompileError;
use super::compiler::{Compiler, Val, MAX_INLINE};
use crate::isa::Op;

impl Compiler {
    /// Compile an expression, returning the register that holds its value.
    pub(super) fn expr(&mut self, e: &Expr) -> Result<Val, CompileError> {
        match e {
            Expr::Num(v) => {
                let d = self.alloc()?;
                self.ops.push(Op::Imm { d, v: Fp::from_u64(*v) });
                Ok(Val { reg: d, temp: true })
            }
            Expr::Var(n) => {
                // A loop variable is a compile-time constant, so it materializes as
                // an immediate. Otherwise the name must resolve to a binding.
                if let Some(v) = self.loop_const(n) {
                    let d = self.alloc()?;
                    self.ops.push(Op::Imm { d, v: Fp::from_u64(v) });
                    return Ok(Val { reg: d, temp: true });
                }
                let reg = self.lookup(n).ok_or(CompileError::UnknownVariable)?;
                Ok(Val { reg, temp: false })
            }
            Expr::Add(l, r) => self.binary(l, r, |d, a, b| Op::Add { d, a, b }),
            Expr::Sub(l, r) => self.binary(l, r, |d, a, b| Op::Sub { d, a, b }),
            Expr::Mul(l, r) => self.binary(l, r, |d, a, b| Op::Mul { d, a, b }),
            Expr::Eq(l, r) => self.binary(l, r, |d, a, b| Op::Eq { d, a, b }),
            // Division is sugar with no opcode of its own: a / b is a * b^{-1}. We
            // invert the divisor, then multiply. Because inverting zero has no valid
            // trace, dividing by zero is unprovable rather than a wrong answer.
            Expr::Div(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                self.release(&b);
                let recip = self.alloc()?;
                self.ops.push(Op::Inv { d: recip, a: b.reg });
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
            // `sel(c, a, b)` and `if c { a } else { b }` are the same select: both
            // arms are evaluated and one is chosen by the boolean condition.
            Expr::Sel(cond, l, r) => self.select(cond, l, r),
            Expr::If(cond, l, r) => self.select(cond, l, r),
            Expr::Call(name, args) => self.call(name, args),
        }
    }

    // The shared shape of a two-operand arithmetic node: compile both operands,
    // release any temporaries so the result can reuse their registers, allocate the
    // result, and emit the op.
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

    // The shared lowering of the select expression: compile the condition and both
    // arms, release the temporaries, and emit one `Sel` opcode.
    fn select(&mut self, cond: &Expr, l: &Expr, r: &Expr) -> Result<Val, CompileError> {
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

    // Inline a function call. The body is compiled in place with the parameters
    // bound to the argument registers, hygienically: the body sees only its
    // parameters and the other functions, never the caller's names or loop
    // variables. Recursion is a compile error, caught by the inline-depth bound,
    // because inlining a recursive call would not terminate.
    fn call(&mut self, name: &str, args: &[Expr]) -> Result<Val, CompileError> {
        let def = self
            .fns
            .iter()
            .find(|f| f.name.as_str() == name)
            .cloned()
            .ok_or(CompileError::UnknownFunction)?;
        if def.params.len() != args.len() {
            return Err(CompileError::ArityMismatch);
        }
        if self.inline_depth >= MAX_INLINE {
            return Err(CompileError::RecursionTooDeep);
        }
        // Compile the arguments in the caller's scope.
        let mut arg_vals: Vec<Val> = Vec::with_capacity(args.len());
        for a in args {
            arg_vals.push(self.expr(a)?);
        }
        // Swap the name scope for one holding only the parameters, compile the body,
        // then restore. The register pool is shared, only the names change.
        let param_scope: Vec<(String, u8)> =
            def.params.iter().zip(&arg_vals).map(|(p, v)| (p.clone(), v.reg)).collect();
        let saved_syms = core::mem::replace(&mut self.syms, param_scope);
        let saved_loops = core::mem::take(&mut self.loop_consts);
        self.inline_depth += 1;
        let result = self.expr(&def.body)?;
        self.inline_depth -= 1;
        self.syms = saved_syms;
        self.loop_consts = saved_loops;
        // The arguments are dead now. Free each temporary, unless the result is that
        // same register (an identity-like body), which we then keep as ours.
        let mut temp = result.temp;
        for v in &arg_vals {
            if v.temp {
                if v.reg == result.reg {
                    temp = true;
                } else {
                    self.free.push(v.reg);
                }
            }
        }
        Ok(Val { reg: result.reg, temp })
    }
}
