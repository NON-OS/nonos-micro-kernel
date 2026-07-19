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

//! The framework trait impls. `Air` gives the engine the trace shape, the periodic
//! wiring columns, the transition over the base field, and the boundary; `AirExt`
//! gives the same transition over the extension field. Both hand off to the one
//! `transition_impl`, so there is a single definition of the constraints.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::air::{Air, AirExt};
use nonos_stark::field::{Fp, Fp2};

use super::layout::*;
use super::step_air::StepAir;
use super::wiring::WireRow;
use crate::isa::REGS;

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
        // The four ports in order: write, then the three read ports. For each and
        // each register, a column that is one on the rows that name that register.
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
        // Public inputs and outputs, bound to the rows that read or expose them.
        bnd.extend_from_slice(&self.public_bindings);
        bnd
    }
}

impl AirExt for StepAir {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
