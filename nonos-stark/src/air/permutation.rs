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

//! A multiset equality argument: prove two sequences are permutations of each
//! other. It runs the grand product `z` where `z_{i+1} = z_i * (a_i + g)/(b_i + g)`
//! for a challenge `g`; if the sequences hold the same multiset the product
//! telescopes to one. This is the primitive a STARK uses for copy constraints
//! and lookups, and it is what lets a value computed in one region of a trace be
//! bound to another region, the cross-region sharing a monolithic recursive
//! verifier needs. The product accumulates over the first half of the trace and
//! its final value is pinned at a checkpoint, so the last row stays free as the
//! engine requires.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

pub struct Permutation {
    log_n: u32,
    a: Vec<Fp>,
    b: Vec<Fp>,
    gamma: Fp,
}

impl Permutation {
    /// Compare two equal-length, power-of-two sequences under challenge `gamma`.
    pub fn new(a: Vec<Fp>, b: Vec<Fp>, gamma: Fp) -> Permutation {
        let log_n = a.len().trailing_zeros();
        Permutation { log_n, a, b, gamma }
    }

    fn len(&self) -> usize {
        1usize << self.log_n
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let z = window[0];
        let z_next = window[1];
        let a = periodic[0];
        let b = periodic[1];
        let sel = periodic[2];
        let g = F::from_base(self.gamma);
        let product = z_next * (b + g) - z * (a + g);
        let carry = z_next - z;
        alloc::vec![sel * product + (F::ONE - sel) * carry]
    }
}

impl AirExt for Permutation {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for Permutation {
    fn log_trace_len(&self) -> u32 {
        // Twice the sequence length: the product region, then a checkpoint and
        // an inert tail so the final row is free.
        self.log_n + 1
    }

    fn trace_width(&self) -> usize {
        1
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        3
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = self.len();
        let total = 1usize << self.log_trace_len();
        let mut a_col = Vec::with_capacity(total);
        let mut b_col = Vec::with_capacity(total);
        let mut sel = Vec::with_capacity(total);
        for r in 0..total {
            let active = r < n;
            a_col.push(if active { self.a[r] } else { Fp::ZERO });
            b_col.push(if active { self.b[r] } else { Fp::ZERO });
            sel.push(if active { Fp::ONE } else { Fp::ZERO });
        }
        alloc::vec![a_col, b_col, sel]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The product starts at one, and after the whole sequence returns to one
        // exactly when the multisets match.
        alloc::vec![(0, 0, Fp::ONE), (0, self.len(), Fp::ONE)]
    }
}
