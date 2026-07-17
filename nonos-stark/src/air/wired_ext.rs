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

//! The money-grade `Wired`: it stacks `AirExt` regions and binds shared values
//! across them with a copy-constraint grand product, proving the compound
//! statement at ~2^-128. This is the join-split vehicle: one note's value is wired
//! from its conservation region to where it is consumed (range, commitment hash).
//! Layout, region transitions, and the grand product come from `fusion`; this adds
//! only the wiring fields, the product-column witness, and the two thin trait impls.

use super::super::field::{Fp, Fp2};
use super::fusion::{self, Stack};
use super::spec::{Air, AirExt};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct WiredExt {
    regions: Vec<Box<dyn AirExt>>,
    stack: Stack,
    wired_cols: Vec<usize>,
    sigma: Vec<usize>,
    beta: Fp,
    gamma: Fp,
}

impl WiredExt {
    pub fn new(
        regions: Vec<Box<dyn AirExt>>,
        wired_cols: Vec<usize>,
        sigma: Vec<usize>,
        beta: Fp,
        gamma: Fp,
    ) -> WiredExt {
        let stack = Stack::of(&regions);
        WiredExt { regions, stack, wired_cols, sigma, beta, gamma }
    }

    fn stride(&self) -> usize {
        self.stack.width + 1
    }

    /// The first grand-product periodic slot: after the selectors and every
    /// region's own periodic columns.
    fn gp_slot(&self) -> usize {
        let region_slots = self.stack.slot_offsets.last().copied().unwrap_or(0)
            + self.regions.last().map(|r| r.periodic_columns().len()).unwrap_or(0);
        self.regions.len() + region_slots
    }

    /// The prover-side per-row product ratio over the wired columns.
    fn ratio(&self, row: &[Fp], r: usize) -> Fp {
        let k = self.wired_cols.len();
        let (b, g) = (self.beta, self.gamma);
        let mut num = Fp::ONE;
        let mut den = Fp::ONE;
        for (j, &col) in self.wired_cols.iter().enumerate() {
            let v = row[col];
            let id = r * k + j;
            num = num * (v + b * Fp::from_u64(id as u64) + g);
            den = den * (v + b * Fp::from_u64(self.sigma[id] as u64) + g);
        }
        num * den.inv()
    }

    /// The witness: regions in the low columns, the grand product in the last.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let stride = self.stride();
        let total = 1usize << self.log_trace_len();
        let mut trace = fusion::place_traces(&self.stack, &self.regions, stride, total, traces);
        let z_col = self.stack.width;
        let span = self.stack.span();
        let mut z = Fp::ONE;
        for r in 0..total {
            trace[r * stride + z_col] = z;
            if r < span {
                let base = r * stride;
                z = z * self.ratio(&trace[base..base + stride], r);
            }
        }
        trace
    }
}

impl Air for WiredExt {
    fn log_trace_len(&self) -> u32 {
        self.stack.log_span + 1
    }

    fn trace_width(&self) -> usize {
        self.stride()
    }

    fn window_size(&self) -> usize {
        self.stack.window
    }

    fn constraint_degree(&self) -> usize {
        let mut d = 1usize;
        for region in &self.regions {
            d = d.max(region.constraint_degree());
        }
        (d + 2).max(self.wired_cols.len() + 2)
    }

    fn num_transition(&self) -> usize {
        let mut n = 0usize;
        for region in &self.regions {
            n = n.max(region.num_transition());
        }
        n + 1
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        let span = self.stack.span();
        let k = self.wired_cols.len();
        let mut cols = fusion::base_periodic(&self.stack, &self.regions, total);
        for j in 0..k {
            let mut id = alloc::vec![Fp::ZERO; total];
            let mut sig = alloc::vec![Fp::ZERO; total];
            for r in 0..span {
                id[r] = Fp::from_u64((r * k + j) as u64);
                sig[r] = Fp::from_u64(self.sigma[r * k + j] as u64);
            }
            cols.push(id);
            cols.push(sig);
        }
        let mut gp_sel = alloc::vec![Fp::ZERO; total];
        for item in gp_sel.iter_mut().take(span) {
            *item = Fp::ONE;
        }
        cols.push(gp_sel);
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            |i, l, p| self.regions[i].transition(l, p),
        );
        let last = self.num_transition() - 1;
        out[last] = fusion::grand_product(
            &self.wired_cols,
            self.beta,
            self.gamma,
            self.stack.width,
            self.stride(),
            self.gp_slot(),
            window,
            periodic,
        );
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = fusion::base_boundary(&self.stack, &self.regions);
        b.push((self.stack.width, 0, Fp::ONE));
        b.push((self.stack.width, self.stack.span(), Fp::ONE));
        b
    }
}

impl AirExt for WiredExt {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            |i, l, p| self.regions[i].transition_ext(l, p),
        );
        let last = self.num_transition() - 1;
        out[last] = fusion::grand_product(
            &self.wired_cols,
            self.beta,
            self.gamma,
            self.stack.width,
            self.stride(),
            self.gp_slot(),
            window,
            periodic,
        );
        out
    }
}
