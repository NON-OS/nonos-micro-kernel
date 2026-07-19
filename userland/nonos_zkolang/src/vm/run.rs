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

//! The run loop: step through the program, one row per instruction, and stop at
//! the halt. A program that never halts is a typed error, not an infinite loop,
//! because the loop is bounded by the instruction list.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::{ProveError, Vm};
use crate::isa::Program;
use crate::trace::{Row, Trace};

impl Vm {
    /// Run `program` on `inputs`, the first `n_public` of which are public. On
    /// success returns the trace plus the public boundary the proof commits to.
    /// The clock is the row index, so the trace is an ordered sequence by
    /// construction.
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
}
