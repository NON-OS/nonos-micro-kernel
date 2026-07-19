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

//! The transition constraints, written once over any field so the base-field
//! composition and the extension-field out-of-domain evaluation share one
//! definition. Each returned value must be zero on every trace row. The groups are
//! selector well-formedness, the clock step, the opcode gates, and the register
//! binding (reads as a linear combination of the register file, writes as a
//! carry-or-update per register).

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Felt;

use super::layout::*;
use super::step_air::StepAir;
use crate::isa::REGS;

impl StepAir {
    /// The transition over any field. `window` is row-major:
    /// `window[k * TRACE_WIDTH + col]` is column `col` of the k-th window row.
    /// `periodic` holds the wiring one-hots evaluated at the current point.
    pub(super) fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
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
        let s_inp = window[S_INP];
        let s_out = window[S_OUT];
        let s_halt = window[S_HALT];
        let a = window[A];
        let b = window[B];
        let c = window[C];
        let d = window[D];
        let imm = window[IMM];
        let aux = window[AUX];
        let next_clk = window[TRACE_WIDTH + CLK];

        let diff = a - b;
        let selector_sum = s_imm
            + s_add
            + s_sub
            + s_mul
            + s_inv
            + s_eq
            + s_sel
            + s_bool
            + s_assert
            + s_inp
            + s_out
            + s_halt;

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
            s_inp * (s_inp - one),
            s_out * (s_out - one),
            s_halt * (s_halt - one),
            // Exactly one selector is set: the row names one opcode.
            selector_sum - one,
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
            // Input: the register takes the immediate, whose value the boundary
            // pins to the committed public input.
            s_inp * (d - imm),
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
