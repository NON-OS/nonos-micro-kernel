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

//! A two-element algebraic permutation, the shape of a hash round: a full x^7
//! S-box on each state element, a linear mix, and a round constant. The round is
//!
//!   x' = x^7 + y + rc0
//!   y' = x + y^7 + rc1
//!
//! A trace of it is a two-column state, so proving a chain is a
//! computational-integrity proof over a multi-element permutation, the same
//! structure a Poseidon or Rescue hash is built from. This is the multi-column,
//! high-degree case: two degree-seven transition constraints over a width-two
//! trace.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec;
use alloc::vec::Vec;

pub struct Permutation2 {
    pub log_t: u32,
    pub rc0: Fp,
    pub rc1: Fp,
    /// The public final state after the whole chain.
    pub out: [Fp; 2],
}

impl Permutation2 {
    fn transition_impl<F: Felt>(&self, window: &[F], _periodic: &[F]) -> Vec<F> {
        let (x, y, x_next, y_next) = (window[0], window[1], window[2], window[3]);
        alloc::vec![
            x_next - (x.pow(7) + y + F::from_base(self.rc0)),
            y_next - (x + y.pow(7) + F::from_base(self.rc1))
        ]
    }
}

impl AirExt for Permutation2 {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for Permutation2 {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn trace_width(&self) -> usize {
        2
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        7
    }

    fn num_transition(&self) -> usize {
        2
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Both state columns at the last row are the public output.
        let last = (1usize << self.log_t) - 1;
        vec![(0, last, self.out[0]), (1, last, self.out[1])]
    }
}
