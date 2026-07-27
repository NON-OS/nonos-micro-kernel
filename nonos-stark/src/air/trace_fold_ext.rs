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

//! The extension fold-consistency gadget: the money-grade counterpart of
//! `TraceFold`. A money-grade FRI folds `Fp2` values under an `Fp2` challenge, so a
//! recursive verifier of such a proof needs a fold gadget over the extension. Each
//! row holds `[beta, a, b]` as three `Fp2` values, laid out as six base columns
//! `[beta.c0, beta.c1, a.c0, a.c1, b.c0, b.c1]`. The transition recomputes the fold
//! `v = (a + b)/2 + beta * (a - b)/(2x)` in `Fp2`, its multiplication expanded with
//! the `X^2 = 7` cross terms, and checks it against the value the next layer
//! carries. `beta` sits in columns zero and one so a wiring engine can force it to
//! equal an `Fp2` challenge a transcript region squeezed. The domain-point inverse
//! and the position bit stay public.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// The extension non-residue: `Fp2 = Fp[X]/(X^2 - W)`, so `(p + q X)(r + s X) =
/// (pr + W qs) + (ps + qr) X`.
const W: u64 = 7;

pub struct TraceFoldExt {
    log_layers: u32,
    n_folds: usize,
    x_inv: Vec<Fp>,
    dir: Vec<bool>,
    final_value: Fp2,
    witness_points: bool,
}

impl TraceFoldExt {
    pub fn new(
        log_layers: u32,
        n_folds: usize,
        x_inv: Vec<Fp>,
        dir: Vec<bool>,
        final_value: Fp2,
    ) -> TraceFoldExt {
        TraceFoldExt { log_layers, n_folds, x_inv, dir, final_value, witness_points: false }
    }

    /// The production form: the per-layer evaluation point, its inverse, and the
    /// position bit ride the trace instead of the periodic columns, so the AIR
    /// carries no per-query data. The layer points are chained in-region: FRI
    /// evaluates layer `m` at `x_m = (shift * omega^(q mod 2^(L-m-1)))^(2^m)` and
    /// folds toward position bit `dir_m = (q mod 2^(L-m-1) >= 2^(L-m-2))`, so
    /// dropping that bit and squaring gives `x_(m+1) = x_m^2 * (-1)^(dir_m)`
    /// (omega^(2^(L-1)) = -1). The transition enforces exactly that square-and-sign
    /// chain, the inverse witness against its point, and the bit; the layer-zero
    /// point is a witness cell the assembly binds to the index-derived product
    /// chain, so every point descends from the authenticated query index.
    pub fn new_witness(
        log_layers: u32,
        n_folds: usize,
        x_inv: Vec<Fp>,
        dir: Vec<bool>,
        final_value: Fp2,
    ) -> TraceFoldExt {
        TraceFoldExt { log_layers, n_folds, x_inv, dir, final_value, witness_points: true }
    }

    /// The witness: one `[beta, a, b]` row per layer as six base columns, the final
    /// layer carrying its pair, the rest padded. The production form appends the
    /// per-layer `[x, x_inv, dir]` the transition chains and checks.
    pub fn trace(&self, beta: &[Fp2], a: &[Fp2], b: &[Fp2]) -> Vec<Fp> {
        let rows = 1usize << self.log_layers;
        let w = self.trace_width();
        let mut trace = alloc::vec![Fp::ZERO; rows * w];
        let mut put = |row: usize, v: [Fp2; 3]| {
            let base = row * w;
            trace[base] = v[0].c0;
            trace[base + 1] = v[0].c1;
            trace[base + 2] = v[1].c0;
            trace[base + 3] = v[1].c1;
            trace[base + 4] = v[2].c0;
            trace[base + 5] = v[2].c1;
        };
        for m in 0..self.n_folds {
            put(m, [beta[m], a[m], b[m]]);
        }
        put(self.n_folds, [Fp2::ZERO, a[self.n_folds], b[self.n_folds]]);
        if self.witness_points {
            for m in 0..self.n_folds {
                let base = m * w;
                trace[base + 6] = self.x_inv[m].inv();
                trace[base + 7] = self.x_inv[m];
                trace[base + 8] = if self.dir[m] { Fp::ONE } else { Fp::ZERO };
            }
        }
        trace
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let stride = self.trace_width();
        let (bc0, bc1) = (window[0], window[1]);
        let (ac0, ac1) = (window[2], window[3]);
        let (bbc0, bbc1) = (window[4], window[5]);
        let (na_c0, na_c1) = (window[stride + 2], window[stride + 3]);
        let (nb_c0, nb_c1) = (window[stride + 4], window[stride + 5]);
        let sel = periodic[0];
        let (x_inv, dir) =
            if self.witness_points { (window[7], window[8]) } else { (periodic[1], periodic[2]) };

        let inv2 = F::from_base(Fp::from_u64(2).inv());
        let w = F::from_base(Fp::from_u64(W));

        // even = (a + b)/2, odd = (a - b)/(2x), componentwise (scaling by base).
        let even_c0 = (ac0 + bbc0) * inv2;
        let even_c1 = (ac1 + bbc1) * inv2;
        let odd_c0 = (ac0 - bbc0) * inv2 * x_inv;
        let odd_c1 = (ac1 - bbc1) * inv2 * x_inv;

        // beta * odd in Fp2: (b0 o0 + W b1 o1, b0 o1 + b1 o0).
        let bo_c0 = bc0 * odd_c0 + w * bc1 * odd_c1;
        let bo_c1 = bc0 * odd_c1 + bc1 * odd_c0;

        let folded_c0 = even_c0 + bo_c0;
        let folded_c1 = even_c1 + bo_c1;

        let expected_c0 = (F::ONE - dir) * na_c0 + dir * nb_c0;
        let expected_c1 = (F::ONE - dir) * na_c1 + dir * nb_c1;

        let mut out = alloc::vec![sel * (folded_c0 - expected_c0), sel * (folded_c1 - expected_c1)];
        if self.witness_points {
            let x = window[6];
            let nx = window[stride + 6];
            let chain_sel = periodic[1];
            let two = F::from_base(Fp::from_u64(2));
            // The witnessed bit is a bit, the witnessed inverse inverts the point,
            // and the next layer's point is the square with the dropped bit's sign.
            out.push(dir * (F::ONE - dir));
            out.push(sel * (x * x_inv - F::ONE));
            out.push(chain_sel * (nx - x * x * (F::ONE - two * dir)));
        }
        out
    }
}

impl AirExt for TraceFoldExt {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for TraceFoldExt {
    fn log_trace_len(&self) -> u32 {
        self.log_layers
    }

    fn trace_width(&self) -> usize {
        if self.witness_points {
            9
        } else {
            6
        }
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The fold multiplies the witnessed challenge, the opened value, and the
        // inverse point, then the selector; the production chain squares the point
        // under the bit and the selector, the same four factors.
        4
    }

    fn num_transition(&self) -> usize {
        // Production adds the bit, the inverse, and the point-chain constraints.
        if self.witness_points {
            5
        } else {
            2
        }
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = 1usize << self.log_layers;
        // Production form: the fold selector and the point-chain selector, which
        // stops one layer short so the chain links consecutive active layers.
        if self.witness_points {
            let mut sel = Vec::with_capacity(n);
            let mut chain = Vec::with_capacity(n);
            for r in 0..n {
                sel.push(if r < self.n_folds { Fp::ONE } else { Fp::ZERO });
                chain.push(if r + 1 < self.n_folds { Fp::ONE } else { Fp::ZERO });
            }
            return alloc::vec![sel, chain];
        }
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
        // opened-value (a) columns.
        alloc::vec![(2, self.n_folds, self.final_value.c0), (3, self.n_folds, self.final_value.c1),]
    }
}
