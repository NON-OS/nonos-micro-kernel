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

//! A FRI fold-consistency gadget whose folding challenge is witnessed in the
//! trace rather than supplied as a public column. Each row holds `[beta, a, b]`:
//! the challenge for that layer and the opened pair `(f(x), f(-x))`. The
//! transition recomputes the fold `v = (a + b)/2 + beta * (a - b)/(2x)` and
//! checks it against the value the next layer carries. Because `beta` sits in
//! column zero, the wiring engine can force it to equal a value produced in
//! another region: this is the fold the monolithic verifier runs in-circuit on
//! exactly the challenge a transcript region squeezed, instead of trusting a
//! public input. The domain point inverse and the position bit stay public, as
//! they follow from the committed domain and the query index.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

pub struct TraceFold {
    log_layers: u32,
    n_folds: usize,
    /// Inverse of the low domain point at each folding layer.
    x_inv: Vec<Fp>,
    /// Whether the folded value lands in the next layer's second slot.
    dir: Vec<bool>,
    /// The committed final-layer value.
    final_value: Fp,
}

impl TraceFold {
    pub fn new(
        log_layers: u32,
        n_folds: usize,
        x_inv: Vec<Fp>,
        dir: Vec<bool>,
        final_value: Fp,
    ) -> TraceFold {
        TraceFold { log_layers, n_folds, x_inv, dir, final_value }
    }

    /// The witness: one `[beta, a, b]` row per layer, the final layer carrying
    /// its pair, the rest padded. `beta[m]` is the challenge at layer `m`, in
    /// column zero so the wiring engine can bind it.
    pub fn trace(&self, beta: &[Fp], a: &[Fp], b: &[Fp]) -> Vec<Fp> {
        let rows = 1usize << self.log_layers;
        let mut trace = alloc::vec![Fp::ZERO; rows * 3];
        for m in 0..self.n_folds {
            trace[m * 3] = beta[m];
            trace[m * 3 + 1] = a[m];
            trace[m * 3 + 2] = b[m];
        }
        // The final layer row: its pair, first slot the committed value.
        trace[self.n_folds * 3 + 1] = a[self.n_folds];
        trace[self.n_folds * 3 + 2] = b[self.n_folds];
        trace
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let beta = window[0];
        let (a, b) = (window[1], window[2]);
        let (next_a, next_b) = (window[4], window[5]);
        let sel = periodic[0];
        let x_inv = periodic[1];
        let dir = periodic[2];

        let inv2 = F::from_base(Fp::from_u64(2).inv());
        let even = (a + b) * inv2;
        let odd = (a - b) * inv2 * x_inv;
        let folded = even + beta * odd;

        let expected = (F::ONE - dir) * next_a + dir * next_b;
        alloc::vec![sel * (folded - expected)]
    }
}

impl AirExt for TraceFold {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for TraceFold {
    fn log_trace_len(&self) -> u32 {
        self.log_layers
    }

    fn trace_width(&self) -> usize {
        3
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The fold multiplies the witnessed challenge, the opened value, and the
        // public inverse point, then the selector: four interpolated factors.
        4
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = 1usize << self.log_layers;
        let mut sel = Vec::with_capacity(n);
        let mut xi = Vec::with_capacity(n);
        let mut dr = Vec::with_capacity(n);
        for r in 0..n {
            let active = r < self.n_folds;
            sel.push(if active { Fp::ONE } else { Fp::ZERO });
            xi.push(if active { self.x_inv[r] } else { Fp::ZERO });
            dr.push(if active && self.dir[r] { Fp::ONE } else { Fp::ZERO });
        }
        alloc::vec![sel, xi, dr]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The layer after the last fold carries the committed final value in the
        // opened-value column.
        alloc::vec![(1, self.n_folds, self.final_value)]
    }
}
