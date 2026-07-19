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

//! The zkolang VM executor. It runs a compiled program on public and private
//! inputs and emits the execution trace the STARK proves. It never panics: a
//! malformed program is a typed error, and a violated constraint (a failed
//! assert, an inverse of zero, a non-boolean selector) is reported as
//! `Unprovable`, the honest result, because such a trace has no proof.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use crate::isa::{Op, Program, REGS};
use crate::trace::{OpTag, Row, Trace};

// Why a run produced no valid trace. `Unprovable` is a legitimate outcome, not
// a bug: the witness did not satisfy the program's constraints.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProveError {
    // A register index outside `0..REGS`.
    BadRegister(u8),
    // An input index past the supplied input vector.
    BadInput(u16),
    // The program ran its whole instruction list without a `Halt`.
    NoHalt,
    // A constraint the trace must satisfy did not hold, at this step.
    Unprovable { step: u64 },
}

// The machine: a fixed register file, a sparse field-addressed memory, and the
// Poseidon permutation the `Pos` opcode uses. `hash` is injected so the core
// stays independent of the concrete permutation while the AIR proves the real
// one used at build.
pub struct Vm {
    regs: [Fp; REGS],
    mem: BTreeMap<u64, Fp>,
    hash: fn(Fp, Fp) -> Fp,
}

impl Vm {
    pub fn new(hash: fn(Fp, Fp) -> Fp) -> Vm {
        Vm { regs: [Fp::ZERO; REGS], mem: BTreeMap::new(), hash }
    }

    // Run `program` on `inputs`, the first `n_public` of which are public. On
    // success returns the trace plus the public boundary the proof commits to.
    pub fn run(
        &mut self,
        program: &Program,
        inputs: &[Fp],
        n_public: usize,
    ) -> Result<Trace, ProveError> {
        let mut rows: Vec<Row> = Vec::with_capacity(program.len());
        let mut outputs: Vec<Fp> = Vec::new();

        for (i, op) in program.iter().enumerate() {
            let clk = i as u64;
            let mut row = Row::at(clk);
            if self.step(*op, inputs, &mut outputs, &mut row, clk)? {
                rows.push(row);
                let n_pub = n_public.min(inputs.len());
                return Ok(Trace {
                    rows,
                    public_inputs: inputs[..n_pub].to_vec(),
                    public_outputs: outputs,
                });
            }
            rows.push(row);
        }
        Err(ProveError::NoHalt)
    }

    // Execute one instruction, filling `row`. Returns Ok(true) on `Halt`.
    fn step(
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
                if va == Fp::ZERO {
                    return Err(ProveError::Unprovable { step: clk });
                }
                let inv = va.inv();
                row.rd = inv;
                row.aux = inv;
                self.wset(d, inv)?;
            }
            Op::Load { d, a } => {
                row.op = OpTag::Load;
                let addr = self.rget(a)?;
                row.ra = addr;
                row.addr = addr;
                let v = *self.mem.get(&addr.value()).unwrap_or(&Fp::ZERO);
                row.mval = v;
                row.rd = v;
                self.wset(d, v)?;
            }
            Op::Store { a, b } => {
                row.op = OpTag::Store;
                let addr = self.rget(a)?;
                let v = self.rget(b)?;
                row.ra = addr;
                row.rb = v;
                row.addr = addr;
                row.mval = v;
                self.mem.insert(addr.value(), v);
            }
            Op::Sel { d, c, a, b } => {
                row.op = OpTag::Sel;
                let vc = self.rget(c)?;
                let va = self.rget(a)?;
                let vb = self.rget(b)?;
                row.rc = vc;
                row.ra = va;
                row.rb = vb;
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
                // aux is the inverse of the difference when non-zero, the EQ
                // witness the AIR uses; the result is 1 exactly when equal.
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
            Op::Pos { d, a, b } => {
                row.op = OpTag::Pos;
                let va = self.rget(a)?;
                let vb = self.rget(b)?;
                let out = (self.hash)(va, vb);
                row.ra = va;
                row.rb = vb;
                row.pos_in0 = va;
                row.pos_in1 = vb;
                row.pos_out = out;
                row.rd = out;
                self.wset(d, out)?;
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

    // Read a register, bounds-checked.
    fn rget(&self, idx: u8) -> Result<Fp, ProveError> {
        self.regs.get(idx as usize).copied().ok_or(ProveError::BadRegister(idx))
    }

    // Write a register, bounds-checked.
    fn wset(&mut self, idx: u8, v: Fp) -> Result<(), ProveError> {
        match self.regs.get_mut(idx as usize) {
            Some(slot) => {
                *slot = v;
                Ok(())
            }
            None => Err(ProveError::BadRegister(idx)),
        }
    }

    // The shared body of Add, Sub, and Mul.
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

// True when a field element is 0 or 1.
fn is_bool(v: Fp) -> bool {
    v == Fp::ZERO || v == Fp::ONE
}
