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

//! One instruction. `step` fills the trace row for a single opcode and updates the
//! register file. The witnessed opcodes (invert, equality) record the auxiliary
//! value the AIR checks, and a violated constraint returns `Unprovable` rather
//! than panicking. `arith` is the shared body of add, subtract, and multiply.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::{ProveError, Vm};
use crate::isa::Op;
use crate::trace::{OpTag, Row};

impl Vm {
    /// Execute one instruction, filling `row`. Returns `Ok(true)` on `Halt`, which
    /// tells the run loop to stop.
    pub(super) fn step(
        &mut self,
        op: Op,
        inputs: &[Fp],
        outputs: &mut Vec<Fp>,
        row: &mut Row,
        clk: u64,
    ) -> Result<bool, ProveError> {
        match op {
            Op::Imm { d, v } => {
                row.op = OpTag::Imm;
                row.imm = v;
                row.rd = v;
                self.wset(d, v)?;
            }
            Op::Add { d, a, b } => self.arith(OpTag::Add, d, a, b, row, |x, y| x + y)?,
            Op::Sub { d, a, b } => self.arith(OpTag::Sub, d, a, b, row, |x, y| x - y)?,
            Op::Mul { d, a, b } => self.arith(OpTag::Mul, d, a, b, row, |x, y| x * y)?,
            Op::Inv { d, a } => {
                row.op = OpTag::Inv;
                let va = self.rget(a)?;
                row.ra = va;
                // Zero has no inverse, so an inversion of zero has no valid trace.
                if va == Fp::ZERO {
                    return Err(ProveError::Unprovable { step: clk });
                }
                let inv = va.inv();
                row.rd = inv;
                row.aux = inv;
                self.wset(d, inv)?;
            }
            Op::Sel { d, c, a, b } => {
                row.op = OpTag::Sel;
                let vc = self.rget(c)?;
                let va = self.rget(a)?;
                let vb = self.rget(b)?;
                row.rc = vc;
                row.ra = va;
                row.rb = vb;
                // The condition must be a bit, or the select has no valid trace.
                if !is_bool(vc) {
                    return Err(ProveError::Unprovable { step: clk });
                }
                let out = if vc == Fp::ONE { va } else { vb };
                row.rd = out;
                self.wset(d, out)?;
            }
            Op::Eq { d, a, b } => {
                row.op = OpTag::Eq;
                let va = self.rget(a)?;
                let vb = self.rget(b)?;
                row.ra = va;
                row.rb = vb;
                let diff = va - vb;
                // aux is the inverse of the difference when non-zero, the equality
                // witness the AIR uses; the result is one exactly when equal.
                let (eq, aux) =
                    if diff == Fp::ZERO { (Fp::ONE, Fp::ZERO) } else { (Fp::ZERO, diff.inv()) };
                row.rd = eq;
                row.aux = aux;
                self.wset(d, eq)?;
            }
            Op::Bool { a } => {
                row.op = OpTag::Bool;
                let va = self.rget(a)?;
                row.ra = va;
                row.aux = va;
                if !is_bool(va) {
                    return Err(ProveError::Unprovable { step: clk });
                }
            }
            Op::Assert { a } => {
                row.op = OpTag::Assert;
                let va = self.rget(a)?;
                row.ra = va;
                row.aux = va;
                if va != Fp::ZERO {
                    return Err(ProveError::Unprovable { step: clk });
                }
            }
            Op::Inp { d, idx } => {
                row.op = OpTag::Inp;
                let v = *inputs.get(idx as usize).ok_or(ProveError::BadInput(idx))?;
                row.imm = v;
                row.rd = v;
                self.wset(d, v)?;
            }
            Op::Out { a, idx } => {
                row.op = OpTag::Out;
                let v = self.rget(a)?;
                row.ra = v;
                let i = idx as usize;
                if outputs.len() <= i {
                    outputs.resize(i + 1, Fp::ZERO);
                }
                outputs[i] = v;
            }
            Op::Halt => {
                row.op = OpTag::Halt;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// The shared body of add, subtract, and multiply: read two registers, record
    /// them, apply the field operation, and write the result.
    fn arith(
        &mut self,
        tag: OpTag,
        d: u8,
        a: u8,
        b: u8,
        row: &mut Row,
        f: fn(Fp, Fp) -> Fp,
    ) -> Result<(), ProveError> {
        row.op = tag;
        let va = self.rget(a)?;
        let vb = self.rget(b)?;
        row.ra = va;
        row.rb = vb;
        let out = f(va, vb);
        row.rd = out;
        self.wset(d, out)
    }
}

/// True when a field element is zero or one, the check the boolean and select
/// opcodes gate on.
fn is_bool(v: Fp) -> bool {
    v == Fp::ZERO || v == Fp::ONE
}
