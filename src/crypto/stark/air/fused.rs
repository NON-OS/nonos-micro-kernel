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

//! Fuse several AIRs into one, so a batch of checks is proven and verified in a
//! single STARK rather than one proof each. The regions are stacked vertically
//! in a trace whose width is the widest region; a per-row selector activates one
//! region's transitions at a time, and each region reads a window repacked to
//! its own width, so heterogeneous widths compose without rewriting the regions.
//! This is the monolithic recursive verifier's backbone: a FRI query verifier is
//! a Merkle-opening region and a fold region fused here, checked at once, and the
//! verification cost stays that of a single STARK regardless of how many regions
//! are folded in.

use super::super::field::Fp;
use super::spec::Air;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Fused {
    regions: Vec<Box<dyn Air>>,
    /// Row offset where each region begins in the fused trace.
    offsets: Vec<usize>,
    /// First periodic slot each region owns, after the selectors.
    slot_offsets: Vec<usize>,
    width: usize,
    window: usize,
    log_len: u32,
}

impl Fused {
    /// Stack `regions` into one trace. Each region keeps its own width and height;
    /// the fused width is the widest region and the fused height is the total row
    /// count rounded to a power of two.
    pub fn new(regions: Vec<Box<dyn Air>>) -> Fused {
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
        let log_len = row.next_power_of_two().trailing_zeros();
        Fused { regions, offsets, slot_offsets, width, window, log_len }
    }

    fn height(&self, i: usize) -> usize {
        1usize << self.regions[i].log_trace_len()
    }

    /// The fused witness: each region's trace laid at its offset, its columns in
    /// the low lanes and the rest zero, padded to the fused height. `traces[i]` is
    /// region `i`'s own row-major trace.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let total = 1usize << self.log_len;
        let mut fused = alloc::vec![Fp::ZERO; total * self.width];
        for (i, region) in self.regions.iter().enumerate() {
            let w = region.trace_width();
            let off = self.offsets[i];
            let h = self.height(i);
            for r in 0..h {
                let base = (off + r) * self.width;
                fused[base..base + w].copy_from_slice(&traces[i][r * w..r * w + w]);
            }
        }
        fused
    }

    /// Number of selector columns: one per region.
    fn selectors(&self) -> usize {
        self.regions.len()
    }
}

impl Air for Fused {
    fn log_trace_len(&self) -> u32 {
        self.log_len
    }

    fn trace_width(&self) -> usize {
        self.width
    }

    fn window_size(&self) -> usize {
        self.window
    }

    fn constraint_degree(&self) -> usize {
        // Each region's constraints are multiplied by its per-row selector, an
        // interpolated column of degree up to the trace length, so the fused
        // composition gains one factor over the widest region. A region may also
        // sit just under its own declared bound after the domain is rounded to a
        // power of two, so two factors of headroom keeps the honest composition
        // below the low-degree bound rather than relying on that rounding.
        let mut d = 1usize;
        for region in &self.regions {
            d = d.max(region.constraint_degree());
        }
        d + 2
    }

    fn num_transition(&self) -> usize {
        let mut n = 0usize;
        for region in &self.regions {
            n = n.max(region.num_transition());
        }
        n
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_len;
        let sel = self.selectors();
        let mut cols: Vec<Vec<Fp>> = Vec::new();
        // Selector columns first: region `i`'s selector is one on its own rows,
        // except its last row, where its window would read into the next region.
        for i in 0..sel {
            let off = self.offsets[i];
            let h = self.height(i);
            let mut col = alloc::vec![Fp::ZERO; total];
            for item in col.iter_mut().take(off + h - 1).skip(off) {
                *item = Fp::ONE;
            }
            cols.push(col);
        }
        // Then each region's own periodic columns, placed at its rows.
        for (i, region) in self.regions.iter().enumerate() {
            let off = self.offsets[i];
            let h = self.height(i);
            for region_col in region.periodic_columns() {
                let mut col = alloc::vec![Fp::ZERO; total];
                col[off..off + h].copy_from_slice(&region_col[..h]);
                cols.push(col);
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let sel = self.selectors();
        let mut out = alloc::vec![Fp::ZERO; self.num_transition()];
        for (i, region) in self.regions.iter().enumerate() {
            let s = periodic[i];
            let w = region.trace_width();
            let ws = region.window_size();
            // Repack the fused window to the region's width so its transition,
            // written for its own layout, reads the right cells.
            let mut local = Vec::with_capacity(w * ws);
            for k in 0..ws {
                local.extend_from_slice(&window[k * self.width..k * self.width + w]);
            }
            let base = sel + self.slot_offsets[i];
            let slots = region.periodic_columns().len();
            let values = region.transition(&local, &periodic[base..base + slots]);
            for (c, v) in values.into_iter().enumerate() {
                out[c] = out[c] + s * v;
            }
        }
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
        b
    }
}
