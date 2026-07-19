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

//! Statement lowering: bindings, assertions, public and private inputs, outputs,
//! and the unrolled `for` loop. A `let` reclaims the register of a binding it
//! shadows when no alias holds it, which is what lets a long accumulator loop fit
//! the register file.

use super::super::parse::{Expr, Stmt};
use super::super::CompileError;
use super::compiler::{Compiler, MAX_UNROLL};
use crate::isa::Op;

impl Compiler {
    /// Lower one statement.
    pub(super) fn stmt(&mut self, s: &Stmt) -> Result<(), CompileError> {
        match s {
            Stmt::Let(name, e) => {
                // Note the register this name held before, if any. The right-hand
                // side may still read it, so we look it up before compiling.
                let old = self.lookup(name);
                let v = self.expr(e)?;
                self.rebind(name, v.reg);
                // If the name shadowed an earlier binding and no other live name
                // holds that register, reclaim it. The alias check is what keeps
                // this sound: a register two names share is never freed under one.
                if let Some(old_reg) = old {
                    if old_reg != v.reg && !self.reg_in_use(old_reg) {
                        self.free.push(old_reg);
                    }
                }
                Ok(())
            }
            Stmt::Assert(e) => self.assert(e),
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
            Stmt::For { var, lo, hi, body } => {
                // Guard against an unreasonable unroll before building anything.
                if hi.saturating_sub(*lo) > MAX_UNROLL {
                    return Err(CompileError::LoopTooLarge);
                }
                // Unroll: for each value, bind the loop variable as a compile-time
                // constant, compile the body inline, then pop the binding. Bodies
                // are flat, so a binding a body makes persists, which is what lets
                // an accumulator across iterations work.
                let mut v = *lo;
                while v < *hi {
                    self.loop_consts.push((var.clone(), v));
                    for s in body {
                        self.stmt(s)?;
                    }
                    self.loop_consts.pop();
                    v += 1;
                }
                Ok(())
            }
        }
    }

    // Lower an `assert`. Writing the comparison out reads naturally: `assert a == b`
    // proves equality by asserting the difference is zero, and `assert a != b`
    // proves inequality by inverting the difference, which succeeds only when it is
    // nonzero. Any other expression is asserted to be zero directly.
    fn assert(&mut self, e: &Expr) -> Result<(), CompileError> {
        match e {
            Expr::Eq(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                self.release(&a);
                self.release(&b);
                let d = self.alloc()?;
                self.ops.push(Op::Sub { d, a: a.reg, b: b.reg });
                self.ops.push(Op::Assert { a: d });
                self.free.push(d);
                Ok(())
            }
            Expr::Ne(l, r) => {
                let a = self.expr(l)?;
                let b = self.expr(r)?;
                self.release(&a);
                self.release(&b);
                let diff = self.alloc()?;
                self.ops.push(Op::Sub { d: diff, a: a.reg, b: b.reg });
                self.free.push(diff);
                // Inverting the difference discards the result; its only job is to
                // fail, and so make the trace unprovable, when the difference is zero.
                let recip = self.alloc()?;
                self.ops.push(Op::Inv { d: recip, a: diff });
                self.free.push(recip);
                Ok(())
            }
            _ => {
                let v = self.expr(e)?;
                self.ops.push(Op::Assert { a: v.reg });
                self.release(&v);
                Ok(())
            }
        }
    }
}
