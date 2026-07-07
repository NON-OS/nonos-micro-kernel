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

//! Fuse several AIRs into one proof and bind values across them. Like the fused
//! engine it stacks regions vertically with a per-row selector, but it adds a
//! grand-product column that runs a Plonk copy constraint over a chosen set of
//! the trace's columns: a public wiring permutation forces chosen cells, in
//! different regions, to hold equal values. This is what the monolithic
//! recursive verifier needs that plain fusion cannot express, and it binds more
//! than one value at once: a fold is forced to run both on the challenge a
//! transcript region squeezed and on the value a Merkle opening revealed, all
//! without laying any of them out adjacently.
//!
//! The wiring runs over `wired_cols`. Each wired cell is numbered `r * k + j` for
//! row `r` and the `j`-th wired column, and `sigma` permutes those numbers; cells
//! in one cycle are forced equal. The regions fill the first half of the trace,
//! the product accumulates over that half and is pinned to one at the midpoint
//! checkpoint exactly when every wiring holds, leaving the tail inert as the
//! engine requires.

use super::super::field::Fp;
use super::spec::Air;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Wired {
    regions: Vec<Box<dyn Air>>,
    /// Row offset where each region begins.
    offsets: Vec<usize>,
    /// First periodic slot each region owns, after the selectors.
    slot_offsets: Vec<usize>,
    /// The trace columns the copy constraint runs over.
    wired_cols: Vec<usize>,
    /// Wiring permutation over the wired cells, numbered `row * k + j` for the
    /// `j`-th wired column. Identity except on the bound cells.
    sigma: Vec<usize>,
    beta: Fp,
    gamma: Fp,
    width: usize,
    window: usize,
    log_span: u32,
}

impl Wired {
    /// Stack `regions` and bind the `wired_cols` under `sigma`. `sigma` is a
    /// permutation over the wired cells (`span * wired_cols.len()` of them, the
    /// `j`-th wired column of row `r` numbered `r * wired_cols.len() + j`); equal
    /// cells sit in the same cycle. `beta` and `gamma` are the copy-constraint
    /// challenges.
    pub fn new(
        regions: Vec<Box<dyn Air>>,
        wired_cols: Vec<usize>,
        sigma: Vec<usize>,
        beta: Fp,
        gamma: Fp,
    ) -> Wired {
        let mut offsets = Vec::with_capacity(regions.len());
        let mut slot_offsets = Vec::with_capacity(regions.len());
        let mut row = 0usize;
        let mut slot = 0usize;
        let mut width = 1usize;
        let mut window = 2usize;
        for region in &regions {
            offsets.push(row);
            slot_offsets.push(slot);
            row += 1usize << region.log_trace_len();
            slot += region.periodic_columns().len();
            width = width.max(region.trace_width());
            window = window.max(region.window_size());
        }
        let log_span = row.next_power_of_two().trailing_zeros();
        Wired {
            regions,
            offsets,
            slot_offsets,
            wired_cols,
            sigma,
            beta,
            gamma,
            width,
            window,
            log_span,
        }
    }

    /// The region span: the first half of the trace, holding the stacked regions
    /// and the running product.
    fn span(&self) -> usize {
        1usize << self.log_span
    }

    fn height(&self, i: usize) -> usize {
        1usize << self.regions[i].log_trace_len()
    }

    /// The full trace width: the widest region plus the running-product column.
    fn stride(&self) -> usize {
        self.width + 1
    }

    /// The per-row contribution to the grand product: over the wired columns,
    /// `prod (cell + beta*id + gamma) / (cell + beta*sigma(id) + gamma)`.
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

    /// The witness: each region's trace at its offset in the low columns, and the
    /// grand product over the wired columns in the last column. `traces[i]` is
    /// region `i`'s own row-major trace.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let stride = self.stride();
        let span = self.span();
        let total = 1usize << self.log_trace_len();
        let z_col = self.width;
        let mut trace = alloc::vec![Fp::ZERO; total * stride];

        for (i, region) in self.regions.iter().enumerate() {
            let w = region.trace_width();
            let off = self.offsets[i];
            let h = self.height(i);
            for r in 0..h {
                let base = (off + r) * stride;
                trace[base..base + w].copy_from_slice(&traces[i][r * w..r * w + w]);
            }
        }

        // The grand product over the wired columns, accumulated across the span.
        let mut z = Fp::ONE;
        for r in 0..total {
            trace[r * stride + z_col] = z;
            if r < span {
                let base = r * stride;
                let ratio = self.ratio(&trace[base..base + stride], r);
                z = z * ratio;
            }
        }
        trace
    }

    fn selectors(&self) -> usize {
        self.regions.len()
    }
}

impl Air for Wired {
    fn log_trace_len(&self) -> u32 {
        // The region span, then a checkpoint and an inert tail so the product
        // closes and the final row stays free.
        self.log_span + 1
    }

    fn trace_width(&self) -> usize {
        self.stride()
    }

    fn window_size(&self) -> usize {
        self.window
    }

    fn constraint_degree(&self) -> usize {
        // The region constraints carry a selector factor, as in the fused engine,
        // with two factors of headroom; the grand product multiplies the product
        // column and one wiring factor per wired column, then its selector.
        let mut d = 1usize;
        for region in &self.regions {
            d = d.max(region.constraint_degree());
        }
        (d + 2).max(self.wired_cols.len() + 2)
    }

    fn num_transition(&self) -> usize {
        // One slot per region constraint, then the grand-product constraint.
        let mut n = 0usize;
        for region in &self.regions {
            n = n.max(region.num_transition());
        }
        n + 1
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        let span = self.span();
        let sel = self.selectors();
        let k = self.wired_cols.len();
        let mut cols: Vec<Vec<Fp>> = Vec::new();

        // Region selectors: one on a region's rows, except its last row.
        for i in 0..sel {
            let off = self.offsets[i];
            let h = self.height(i);
            let mut col = alloc::vec![Fp::ZERO; total];
            for item in col.iter_mut().take(off + h - 1).skip(off) {
                *item = Fp::ONE;
            }
            cols.push(col);
        }
        // Region periodic columns at their offsets.
        for (i, region) in self.regions.iter().enumerate() {
            let off = self.offsets[i];
            let h = self.height(i);
            for region_col in region.periodic_columns() {
                let mut col = alloc::vec![Fp::ZERO; total];
                col[off..off + h].copy_from_slice(&region_col[..h]);
                cols.push(col);
            }
        }
        // Grand-product columns: for each wired column its cell identity and
        // wiring, then the product selector, all live only over the span.
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
        let sel = self.selectors();
        let stride = self.stride();
        let k = self.wired_cols.len();
        let region_slots: usize = self.slot_offsets.last().copied().unwrap_or(0)
            + self.regions.last().map(|r| r.periodic_columns().len()).unwrap_or(0);
        let mut out = alloc::vec![Fp::ZERO; self.num_transition()];

        // Region constraints, selected per row and repacked to each region width.
        for (i, region) in self.regions.iter().enumerate() {
            let s = periodic[i];
            let w = region.trace_width();
            let ws = region.window_size();
            let mut local = Vec::with_capacity(w * ws);
            for step in 0..ws {
                local.extend_from_slice(&window[step * stride..step * stride + w]);
            }
            let base = sel + self.slot_offsets[i];
            let slots = region.periodic_columns().len();
            let values = region.transition(&local, &periodic[base..base + slots]);
            for (c, v) in values.into_iter().enumerate() {
                out[c] = out[c] + s * v;
            }
        }

        // The grand product over the wired columns.
        let gp = sel + region_slots;
        let (b, g) = (self.beta, self.gamma);
        let z = window[self.width];
        let z_next = window[stride + self.width];
        let mut num = Fp::ONE;
        let mut den = Fp::ONE;
        for (j, &col) in self.wired_cols.iter().enumerate() {
            let v = window[col];
            let id = periodic[gp + 2 * j];
            let sig = periodic[gp + 2 * j + 1];
            num = num * (v + b * id + g);
            den = den * (v + b * sig + g);
        }
        let gp_sel = periodic[gp + 2 * k];
        let product = z_next * den - z * num;
        let carry = z_next - z;
        let last = self.num_transition() - 1;
        out[last] = gp_sel * product + (Fp::ONE - gp_sel) * carry;
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = Vec::new();
        for (i, region) in self.regions.iter().enumerate() {
            let off = self.offsets[i];
            for (col, row, val) in region.boundary() {
                b.push((col, off + row, val));
            }
        }
        // The product starts at one and closes to one at the checkpoint exactly
        // when every wired cell agrees.
        b.push((self.width, 0, Fp::ONE));
        b.push((self.width, self.span(), Fp::ONE));
        b
    }
}
