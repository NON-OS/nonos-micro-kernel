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

//! The proeve step AIR: the branchless computational core with register binding.
//!
//! This AIR proves, for a trace laid out one VM step per row:
//!
//!   1. every row carries exactly one opcode (the selectors are boolean and
//!      sum to one), so no row is ambiguous or opcode-free;
//!   2. every row's result equals the operation its selector names: field add,
//!      subtract, multiply, and invert; an equality test that yields a clean bit;
//!      a conditional select; and the two constraint opcodes, a boolean check and
//!      a zero assertion, that let a program state a fact the proof must uphold;
//!   3. every operand a row reads is the live value of the register it names, and
//!      every register carries its value forward unchanged until the row that
//!      writes it, at which point it takes that row's result;
//!   4. the rows are clock-ordered, the counter rising by one each step;
//!   5. a halt row, which is also every padding row up to the power-of-two
//!      length, carries no data in its operand columns.
//!
//! Register binding, point three, is what makes this a proof that a program ran
//! rather than a proof that a bag of individually valid rows exists. proeve names
//! its registers by compile-time index, so the data flow, which row's result
//! feeds which row's operand, is a public property of the program, not of the
//! witness. The AIR carries it as periodic one-hot columns: for each row, which
//! register it writes and which registers its three read ports name. The register
//! file itself is threaded through the trace as `REGS` columns holding the state
//! before each row. A read is then `operand = sum_k read_onehot_k * regfile_k`
//! and a write is `regfile_next_k = (1 - write_onehot_k) * regfile_k +
//! write_onehot_k * result`, both linear in the trace because the one-hots are
//! public constants. The circuit is public, the register values are the witness.
//!
//! Still deferred: the memory opcodes (Load, Store), the Poseidon port (Pos), and
//! public I/O (Inp, Out). Those are the next phases, and they build on this
//! settled register-bound trace.
//!
//! The transition is written once over the `Felt` abstraction so the base-field
//! composition and the extension-field out-of-domain evaluation share one
//! definition, exactly as the framework's own AIRs do.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::air::{Air, AirExt};
use nonos_stark::field::{Felt, Fp, Fp2};

use crate::isa::{Op, Program, REGS};
use crate::trace::{OpTag, Trace};

// Step column layout. One clock counter, ten one-hot opcode selectors, three read
// operands, one result, one immediate, one auxiliary witness.
const CLK: usize = 0;
const S_IMM: usize = 1;
const S_ADD: usize = 2;
const S_SUB: usize = 3;
const S_MUL: usize = 4;
const S_INV: usize = 5;
const S_EQ: usize = 6;
const S_SEL: usize = 7;
const S_BOOL: usize = 8;
const S_ASSERT: usize = 9;
const S_HALT: usize = 10;
const A: usize = 11;
const B: usize = 12;
const C: usize = 13;
const D: usize = 14;
const IMM: usize = 15;
const AUX: usize = 16;

// The register file occupies the columns after the step columns: `REGS` columns
// holding the register state before the row executes.
const RF_BASE: usize = 17;

/// The width of the step trace: the step columns plus the register file.
pub const TRACE_WIDTH: usize = RF_BASE + REGS;

// Periodic (public) wiring columns, in this order: the write one-hot, then the
// three read-port one-hots, each `REGS` wide.
const WRITE_P: usize = 0;
const READA_P: usize = REGS;
const READB_P: usize = 2 * REGS;
const READC_P: usize = 3 * REGS;
const NUM_PERIODIC: usize = 4 * REGS;

// The window is a row and its successor, so the ordering and write-propagation
// constraints can read the next row.
const WINDOW: usize = 2;

// Transition constraint count: 24 step constraints, three read bindings, and one
// write propagation per register.
const NUM_TRANSITION: usize = 24 + 3 + REGS;

// Highest degree among the constraints, e.g. the multiply gate or the witnessed
// equality `s_eq * (d + diff*aux - 1)`. The register bindings are linear.
const DEGREE: usize = 3;

/// Why a program or VM trace could not be laid out for the step AIR.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BuildError {
    /// A row used an opcode outside the scope this AIR proves. Memory (Load,
    /// Store), the Poseidon port (Pos), and public I/O (Inp, Out) arrive with
    /// later phases.
    NotInScope(OpTag),
    /// The program has no reachable halt, so its length is undefined.
    NoHalt,
    /// The run is longer than the requested power-of-two trace length.
    TooLong { rows: usize, cap: usize },
}

// The data flow of one row: which register it writes, and which registers its
// three read ports name. A `None` port reads nothing and pins its operand to
// zero.
#[derive(Clone, Copy)]
struct WireRow {
    write: Option<u8>,
    read_a: Option<u8>,
    read_b: Option<u8>,
    read_c: Option<u8>,
}

impl WireRow {
    const EMPTY: WireRow = WireRow { write: None, read_a: None, read_b: None, read_c: None };

    // The wiring an opcode induces. Out-of-scope and halt rows wire nothing; a
    // trace that reaches them is rejected by `build_trace`, not here.
    fn of(op: &Op) -> WireRow {
        match *op {
            Op::Imm { d, .. } => WireRow { write: Some(d), ..WireRow::EMPTY },
            Op::Add { d, a, b } | Op::Sub { d, a, b } | Op::Mul { d, a, b } => {
                WireRow { write: Some(d), read_a: Some(a), read_b: Some(b), read_c: None }
            }
            Op::Inv { d, a } => WireRow { write: Some(d), read_a: Some(a), ..WireRow::EMPTY },
            Op::Eq { d, a, b } => {
                WireRow { write: Some(d), read_a: Some(a), read_b: Some(b), read_c: None }
            }
            Op::Sel { d, c, a, b } => {
                WireRow { write: Some(d), read_a: Some(a), read_b: Some(b), read_c: Some(c) }
            }
            Op::Bool { a } | Op::Assert { a } => WireRow { read_a: Some(a), ..WireRow::EMPTY },
            _ => WireRow::EMPTY,
        }
    }
}

/// The step AIR over a trace of `2^log_t` rows, carrying the public data-flow
/// wiring of the program it proves.
pub struct StepAir {
    log_t: u32,
    wiring: Vec<WireRow>,
}

impl StepAir {
    /// Compile a program's public data flow into the AIR. The wiring runs up to
    /// and including the first halt and is padded with wireless rows to the
    /// power-of-two length, matching how the VM stops at halt and `build_trace`
    /// pads. The verifier reconstructs the same AIR from the same public program.
    pub fn compile(program: &Program, log_t: u32) -> Result<StepAir, BuildError> {
        let t = 1usize << log_t;
        let mut wiring: Vec<WireRow> = Vec::new();
        let mut halted = false;
        for op in program.iter() {
            wiring.push(WireRow::of(op));
            if matches!(op, Op::Halt) {
                halted = true;
                break;
            }
        }
        if !halted {
            return Err(BuildError::NoHalt);
        }
        if wiring.len() > t {
            return Err(BuildError::TooLong { rows: wiring.len(), cap: t });
        }
        while wiring.len() < t {
            wiring.push(WireRow::EMPTY);
        }
        Ok(StepAir { log_t, wiring })
    }

    /// Lay a VM run out in the step column format, replay the register file into
    /// its columns, and pad with halt rows to the power-of-two length. The clock
    /// column is the row index, so ordering holds across the padding as well as
    /// the run. An out-of-scope opcode is a typed error rather than a silent drop.
    pub fn build_trace(&self, trace: &Trace) -> Result<Vec<Fp>, BuildError> {
        let t = 1usize << self.log_t;
        let n = trace.rows.len();
        if n > t {
            return Err(BuildError::TooLong { rows: n, cap: t });
        }
        let mut flat = vec![Fp::ZERO; t * TRACE_WIDTH];
        let mut regfile = [Fp::ZERO; REGS];
        for i in 0..t {
            let base = i * TRACE_WIDTH;
            flat[base + CLK] = Fp::from_u64(i as u64);
            if i < n {
                Self::write_step_columns(&mut flat, base, &trace.rows[i])?;
            } else {
                // Padding: a clean halt row.
                flat[base + S_HALT] = Fp::ONE;
            }
            // Record the register file state before this row executes.
            for (k, value) in regfile.iter().enumerate() {
                flat[base + RF_BASE + k] = *value;
            }
            // Then apply this row's write, so the next row sees the update.
            if i < n {
                if let Some(k) = self.wiring[i].write {
                    regfile[k as usize] = trace.rows[i].rd;
                }
            }
        }
        Ok(flat)
    }

    // Fill the step columns of one row from a VM row. Register binding is handled
    // by the caller, which threads the register file separately.
    fn write_step_columns(
        flat: &mut [Fp],
        base: usize,
        row: &crate::trace::Row,
    ) -> Result<(), BuildError> {
        match row.op {
            OpTag::Imm => {
                flat[base + S_IMM] = Fp::ONE;
                flat[base + D] = row.rd;
                flat[base + IMM] = row.imm;
            }
            OpTag::Add => {
                flat[base + S_ADD] = Fp::ONE;
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + D] = row.rd;
            }
            OpTag::Sub => {
                flat[base + S_SUB] = Fp::ONE;
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + D] = row.rd;
            }
            OpTag::Mul => {
                flat[base + S_MUL] = Fp::ONE;
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + D] = row.rd;
            }
            OpTag::Inv => {
                flat[base + S_INV] = Fp::ONE;
                flat[base + A] = row.ra;
                flat[base + D] = row.rd;
                flat[base + AUX] = row.aux;
            }
            OpTag::Eq => {
                flat[base + S_EQ] = Fp::ONE;
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + D] = row.rd;
                flat[base + AUX] = row.aux;
            }
            OpTag::Sel => {
                flat[base + S_SEL] = Fp::ONE;
                flat[base + A] = row.ra;
                flat[base + B] = row.rb;
                flat[base + C] = row.rc;
                flat[base + D] = row.rd;
            }
            OpTag::Bool => {
                flat[base + S_BOOL] = Fp::ONE;
                flat[base + A] = row.ra;
            }
            OpTag::Assert => {
                flat[base + S_ASSERT] = Fp::ONE;
                flat[base + A] = row.ra;
            }
            OpTag::Halt => {
                flat[base + S_HALT] = Fp::ONE;
            }
            other => return Err(BuildError::NotInScope(other)),
        }
        Ok(())
    }

    // The transition written once over any field. `window` is row-major:
    // `window[k * TRACE_WIDTH + col]` is column `col` of the k-th window row.
    // `periodic` holds the wiring one-hots evaluated at the current point.
    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let one = F::ONE;

        let clk = window[CLK];
        let s_imm = window[S_IMM];
        let s_add = window[S_ADD];
        let s_sub = window[S_SUB];
        let s_mul = window[S_MUL];
        let s_inv = window[S_INV];
        let s_eq = window[S_EQ];
        let s_sel = window[S_SEL];
        let s_bool = window[S_BOOL];
        let s_assert = window[S_ASSERT];
        let s_halt = window[S_HALT];
        let a = window[A];
        let b = window[B];
        let c = window[C];
        let d = window[D];
        let imm = window[IMM];
        let aux = window[AUX];
        let next_clk = window[TRACE_WIDTH + CLK];

        let diff = a - b;

        let mut cs = vec![
            // Each selector is boolean.
            s_imm * (s_imm - one),
            s_add * (s_add - one),
            s_sub * (s_sub - one),
            s_mul * (s_mul - one),
            s_inv * (s_inv - one),
            s_eq * (s_eq - one),
            s_sel * (s_sel - one),
            s_bool * (s_bool - one),
            s_assert * (s_assert - one),
            s_halt * (s_halt - one),
            // Exactly one selector is set: the row names one opcode.
            s_imm + s_add + s_sub + s_mul + s_inv + s_eq + s_sel + s_bool + s_assert + s_halt - one,
            // The clock rises by one, fixing the row order.
            next_clk - clk - one,
            // Arithmetic: the result is the field operation on the operands.
            s_imm * (d - imm),
            s_add * (d - (a + b)),
            s_sub * (d - (a - b)),
            s_mul * (d - a * b),
            // Invert: aux is a^{-1}, forcing a nonzero, and the result equals it.
            s_inv * (a * aux - one),
            s_inv * (d - aux),
            // Equality: d is one exactly when a == b. If they differ, d*diff = 0
            // forces d = 0 and aux = diff^{-1}; if equal, d = 1.
            s_eq * (d * diff),
            s_eq * (d + diff * aux - one),
            // Select: c is boolean and d = c ? a : b, written c*a + b - c*b.
            s_sel * (c * (c - one)),
            s_sel * (d - (c * a + b - c * b)),
            // Constraint opcodes: a is boolean, or a is zero.
            s_bool * (a * (a - one)),
            s_assert * a,
        ];

        // Register binding. Each read port equals the register it names; each
        // register carries forward unless this row writes it.
        let mut read_a = F::ZERO;
        let mut read_b = F::ZERO;
        let mut read_c = F::ZERO;
        for k in 0..REGS {
            let rf_k = window[RF_BASE + k];
            read_a = read_a + periodic[READA_P + k] * rf_k;
            read_b = read_b + periodic[READB_P + k] * rf_k;
            read_c = read_c + periodic[READC_P + k] * rf_k;
        }
        cs.push(a - read_a);
        cs.push(b - read_b);
        cs.push(c - read_c);
        for k in 0..REGS {
            let rf_k = window[RF_BASE + k];
            let rf_next_k = window[TRACE_WIDTH + RF_BASE + k];
            let w_k = periodic[WRITE_P + k];
            cs.push(rf_next_k - ((one - w_k) * rf_k + w_k * d));
        }

        cs
    }
}

impl Air for StepAir {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn trace_width(&self) -> usize {
        TRACE_WIDTH
    }

    fn window_size(&self) -> usize {
        WINDOW
    }

    fn constraint_degree(&self) -> usize {
        DEGREE
    }

    fn num_transition(&self) -> usize {
        NUM_TRANSITION
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let t = 1usize << self.log_t;
        let mut cols: Vec<Vec<Fp>> = Vec::with_capacity(NUM_PERIODIC);
        let port = |w: &WireRow, which: usize| -> Option<u8> {
            match which {
                0 => w.write,
                1 => w.read_a,
                2 => w.read_b,
                _ => w.read_c,
            }
        };
        for which in 0..4 {
            for k in 0..REGS {
                let mut col = vec![Fp::ZERO; t];
                for (i, cell) in col.iter_mut().enumerate() {
                    if port(&self.wiring[i], which) == Some(k as u8) {
                        *cell = Fp::ONE;
                    }
                }
                cols.push(col);
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let last = (1usize << self.log_t) - 1;
        let mut bnd = vec![
            // The clock starts at zero.
            (CLK, 0, Fp::ZERO),
            // The final row is a clean halt, which the last-row transition gap
            // cannot otherwise reach, so the tail carries no operand data.
            (S_HALT, last, Fp::ONE),
            (A, last, Fp::ZERO),
            (B, last, Fp::ZERO),
            (C, last, Fp::ZERO),
            (D, last, Fp::ZERO),
            (IMM, last, Fp::ZERO),
            (AUX, last, Fp::ZERO),
        ];
        // Every register starts at zero.
        for k in 0..REGS {
            bnd.push((RF_BASE + k, 0, Fp::ZERO));
        }
        bnd
    }
}

impl AirExt for StepAir {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
