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

//! Deriving a query evaluation point from its index in-circuit. A recursive verifier
//! evaluates the DEEP quotient and folds FRI at `x = shift * omega^p`, and `x` is not
//! a copy of any committed cell: it is a power of the index `p`. Left as a free input
//! a prover would choose the evaluation point and forge the quotient, so it must be
//! computed from the bound index. This gadget runs the product chain `x = shift *
//! prod_k (omega^(2^k))^(bit_k)`: one row per index bit, multiplying in the structural
//! constant `omega^(2^k)` exactly when bit `k` is set. The bits ride the trace (the
//! assembly binds them to the opening directions that already carry `p`), the
//! constants are periodic, and only the initial `shift` is a boundary, so the AIR is
//! instance-independent. The final point is witness, bound to where the DEEP check
//! consumes it.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

pub struct IndexPoint {
    log_len: u32,
    bits: usize,
    consts: Vec<Fp>,
    shift: Fp,
    index: usize,
}

impl IndexPoint {
    /// The chain for `shift * omega^index` over `bits` index bits.
    pub fn new(omega: Fp, shift: Fp, bits: usize, index: usize) -> IndexPoint {
        let log_len = (bits + 1).next_power_of_two().trailing_zeros();
        let consts: Vec<Fp> = (0..bits).map(|k| omega.pow(1u64 << k)).collect();
        IndexPoint { log_len, bits, consts, shift, index }
    }

    /// The derived point, for the caller to bind against.
    pub fn point(&self) -> Fp2 {
        let mut x = Fp2::from_base(self.shift);
        for (k, c) in self.consts.iter().enumerate() {
            if (self.index >> k) & 1 == 1 {
                x = x * Fp2::from_base(*c);
            }
        }
        x
    }

    /// The row the final point sits on (after the last bit), for the binding.
    pub fn point_row(&self) -> usize {
        self.bits
    }

    /// The witness: per row the current bit and the running point; the point after
    /// the last bit is the answer, padding rows carry it.
    pub fn trace(&self) -> Vec<Fp> {
        let rows = 1usize << self.log_len;
        let mut tr = alloc::vec![Fp::ZERO; rows * 3];
        let mut x = Fp2::from_base(self.shift);
        for r in 0..rows {
            let b = if r < self.bits && (self.index >> r) & 1 == 1 { Fp::ONE } else { Fp::ZERO };
            tr[r * 3] = b;
            tr[r * 3 + 1] = x.c0;
            tr[r * 3 + 2] = x.c1;
            let c = if r < self.bits { self.consts[r] } else { Fp::ONE };
            let scalar = if b == Fp::ONE { c } else { Fp::ONE };
            x = x * Fp2::from_base(scalar);
        }
        tr
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let b = window[0];
        let (x0, x1) = (window[1], window[2]);
        let (nx0, nx1) = (window[4], window[5]);
        let c = periodic[0];
        let one = F::ONE;
        // scalar = 1 + b*(c - 1): the constant when the bit is set, one otherwise.
        // The point scales by a base value, so both lanes multiply without a cross
        // term. On padding rows c is one, so the point carries regardless of the bit.
        let scalar = one + b * (c - one);
        alloc::vec![nx0 - x0 * scalar, nx1 - x1 * scalar, b * (one - b)]
    }
}

impl AirExt for IndexPoint {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for IndexPoint {
    fn log_trace_len(&self) -> u32 {
        self.log_len
    }

    fn trace_width(&self) -> usize {
        3
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The point times the bit times the constant: three factors.
        3
    }

    fn num_transition(&self) -> usize {
        3
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let rows = 1usize << self.log_len;
        let mut c = alloc::vec![Fp::ONE; rows];
        for (r, item) in c.iter_mut().enumerate().take(self.bits) {
            *item = self.consts[r];
        }
        alloc::vec![c]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Only the initial point is structural: x_0 = shift. The index bits and the
        // final point are witness, bound by the assembly.
        alloc::vec![(1, 0, self.shift), (2, 0, Fp::ZERO)]
    }
}
