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

//! Laying a run out as the flat trace matrix. Each VM row becomes one AIR row: the
//! clock, the opcode's step columns, and the register file recorded before the
//! row, then the row's write applied so the next row sees it. Padding rows are
//! clean halts.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::error::BuildError;
use super::layout::*;
use super::step_air::StepAir;
use crate::isa::REGS;
use crate::trace::{OpTag, Row, Trace};

impl StepAir {
    /// Lay a VM run out in the step column format, replay the register file into
    /// its columns, and pad with halt rows to the power-of-two length. The clock
    /// column is the row index, so ordering holds across the padding as well as
    /// the run.
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
                Self::write_step_columns(&mut flat, base, &trace.rows[i]);
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
    fn write_step_columns(flat: &mut [Fp], base: usize, row: &Row) {
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
            OpTag::Inp => {
                flat[base + S_INP] = Fp::ONE;
                flat[base + D] = row.rd;
                flat[base + IMM] = row.imm;
            }
            OpTag::Out => {
                flat[base + S_OUT] = Fp::ONE;
                flat[base + A] = row.ra;
            }
            OpTag::Halt => {
                flat[base + S_HALT] = Fp::ONE;
            }
        }
    }
}
