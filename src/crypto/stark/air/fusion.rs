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

//! Shared region-stacking for the money-grade fused and wired engines, so the
//! offset layout, the selected region-transition combination, and the copy
//! constraint grand product live in one place instead of being copied per engine.
//! The transition helpers are generic over `Felt`, so a single implementation
//! serves both the base-field composition and the extension out-of-domain
//! evaluation; the caller passes a closure that reads each region's constraints.

use super::super::field::{Felt, Fp};
use super::spec::AirExt;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// The computed placement of a stack of regions in one trace.
pub(super) struct Stack {
    pub offsets: Vec<usize>,
    pub slot_offsets: Vec<usize>,
    pub width: usize,
    pub window: usize,
    pub log_span: u32,
}

impl Stack {
    /// Lay regions out end to end, tracking each one's row offset and periodic-slot
    /// offset, the widest width, the largest window, and the rounded total height.
    pub fn of(regions: &[Box<dyn AirExt>]) -> Stack {
        let mut offsets = Vec::with_capacity(regions.len());
        let mut slot_offsets = Vec::with_capacity(regions.len());
        let mut row = 0usize;
        let mut slot = 0usize;
        let mut width = 1usize;
        let mut window = 2usize;
        for region in regions {
            offsets.push(row);
            slot_offsets.push(slot);
            row += 1usize << region.log_trace_len();
            slot += region.periodic_columns().len();
            width = width.max(region.trace_width());
            window = window.max(region.window_size());
        }
        let log_span = row.next_power_of_two().trailing_zeros();
        Stack { offsets, slot_offsets, width, window, log_span }
    }

    pub fn span(&self) -> usize {
        1usize << self.log_span
    }

    pub fn height(regions: &[Box<dyn AirExt>], i: usize) -> usize {
        1usize << regions[i].log_trace_len()
    }
}

/// Combine the selected region transitions into the fused constraint vector.
/// `stride` is the trace width (plus one when a product column follows), and
/// `region_transition` reads region `i`'s constraints over the field `F`, so the
/// same body serves `transition` (base) and `transition_ext` (extension).
pub(super) fn combine<F: Felt>(
    stack: &Stack,
    regions: &[Box<dyn AirExt>],
    num_transition: usize,
    stride: usize,
    window: &[F],
    periodic: &[F],
    region_transition: impl Fn(usize, &[F], &[F]) -> Vec<F>,
) -> Vec<F> {
    let sel = regions.len();
    let mut out = alloc::vec![F::ZERO; num_transition];
    for (i, region) in regions.iter().enumerate() {
        let s = periodic[i];
        let w = region.trace_width();
        let ws = region.window_size();
        let mut local = Vec::with_capacity(w * ws);
        for step in 0..ws {
            local.extend_from_slice(&window[step * stride..step * stride + w]);
        }
        let base = sel + stack.slot_offsets[i];
        let slots = region.periodic_columns().len();
        let values = region_transition(i, &local, &periodic[base..base + slots]);
        for (c, v) in values.into_iter().enumerate() {
            out[c] = out[c] + s * v;
        }
    }
    out
}

/// The copy-constraint grand-product transition value over `F`: in the product
/// region the running product `z` accumulates `(v + beta*id + gamma)` against
/// `(v + beta*sigma + gamma)` over the wired columns; elsewhere it is carried.
/// `gp` is the first grand-product periodic slot.
#[allow(clippy::too_many_arguments)]
pub(super) fn grand_product<F: Felt>(
    wired_cols: &[usize],
    beta: Fp,
    gamma: Fp,
    width: usize,
    stride: usize,
    gp: usize,
    window: &[F],
    periodic: &[F],
) -> F {
    let b = F::from_base(beta);
    let g = F::from_base(gamma);
    let z = window[width];
    let z_next = window[stride + width];
    let mut num = F::ONE;
    let mut den = F::ONE;
    for (j, &col) in wired_cols.iter().enumerate() {
        let v = window[col];
        let id = periodic[gp + 2 * j];
        let sig = periodic[gp + 2 * j + 1];
        num = num * (v + b * id + g);
        den = den * (v + b * sig + g);
    }
    let gp_sel = periodic[gp + 2 * wired_cols.len()];
    let product = z_next * den - z * num;
    let carry = z_next - z;
    gp_sel * product + (F::ONE - gp_sel) * carry
}

/// The selector and region periodic columns shared by both engines: one selector
/// per region (on its rows but its last), then each region's own periodic columns
/// at its offset. Grand-product columns, if any, are appended by the caller.
pub(super) fn base_periodic(
    stack: &Stack,
    regions: &[Box<dyn AirExt>],
    total: usize,
) -> Vec<Vec<Fp>> {
    let mut cols: Vec<Vec<Fp>> = Vec::new();
    for i in 0..regions.len() {
        let off = stack.offsets[i];
        let h = Stack::height(regions, i);
        let mut col = alloc::vec![Fp::ZERO; total];
        for item in col.iter_mut().take(off + h - 1).skip(off) {
            *item = Fp::ONE;
        }
        cols.push(col);
    }
    for (i, region) in regions.iter().enumerate() {
        let off = stack.offsets[i];
        let h = Stack::height(regions, i);
        for region_col in region.periodic_columns() {
            let mut col = alloc::vec![Fp::ZERO; total];
            col[off..off + h].copy_from_slice(&region_col[..h]);
            cols.push(col);
        }
    }
    cols
}

/// Each region's boundaries lifted to their fused rows.
pub(super) fn base_boundary(stack: &Stack, regions: &[Box<dyn AirExt>]) -> Vec<(usize, usize, Fp)> {
    let mut b = Vec::new();
    for (i, region) in regions.iter().enumerate() {
        let off = stack.offsets[i];
        for (col, row, val) in region.boundary() {
            b.push((col, off + row, val));
        }
    }
    b
}

/// The fused witness: each region's row-major trace placed at its offset in the
/// low `stride` columns. `traces[i]` is region `i`'s own trace.
pub(super) fn place_traces(
    stack: &Stack,
    regions: &[Box<dyn AirExt>],
    stride: usize,
    total: usize,
    traces: &[Vec<Fp>],
) -> Vec<Fp> {
    let mut fused = alloc::vec![Fp::ZERO; total * stride];
    for (i, region) in regions.iter().enumerate() {
        let w = region.trace_width();
        let off = stack.offsets[i];
        let h = Stack::height(regions, i);
        for r in 0..h {
            let base = (off + r) * stride;
            fused[base..base + w].copy_from_slice(&traces[i][r * w..r * w + w]);
        }
    }
    fused
}
