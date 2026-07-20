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

//! The money-grade wired engine with the copy constraint split across several
//! grand-product columns. Routing every shared value through one permutation makes
//! that constraint's degree the number of wired columns, which for a large assembly
//! blows up the evaluation domain and the on-chain composition check. Splitting the
//! bindings into independent groups, one grand-product column each, keeps every
//! constraint at the size of its group plus a constant, so the AIR degree stays at
//! the region maximum. Same soundness, same bindings, cheap to verify. Layout,
//! region transitions, and each group's product come from `fusion`.

use super::super::field::{Felt, Fp, Fp2};
use super::fusion::{self, Stack};
use super::spec::{Air, AirExt};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// One copy-constraint group: the columns it binds, the permutation over their
/// cells, and its challenges. Each group is an independent grand product.
pub struct GpGroup {
    pub wired_cols: Vec<usize>,
    pub sigma: Vec<usize>,
    pub beta: Fp,
    pub gamma: Fp,
}

pub struct WiredMultiExt {
    regions: Vec<Box<dyn AirExt>>,
    stack: Stack,
    groups: Vec<GpGroup>,
    group_slots: Vec<usize>,
    region_transitions: usize,
}

impl WiredMultiExt {
    pub fn new(regions: Vec<Box<dyn AirExt>>, groups: Vec<GpGroup>) -> WiredMultiExt {
        let stack = Stack::of(&regions);
        let region_slots = stack.slot_offsets.last().copied().unwrap_or(0)
            + regions.last().map(|r| r.periodic_columns().len()).unwrap_or(0);
        let base = regions.len() + region_slots;
        let mut group_slots = Vec::with_capacity(groups.len());
        let mut s = base;
        for grp in &groups {
            group_slots.push(s);
            s += 2 * grp.wired_cols.len() + 1;
        }
        let mut region_transitions = 0usize;
        for region in &regions {
            region_transitions = region_transitions.max(region.num_transition());
        }
        WiredMultiExt { regions, stack, groups, group_slots, region_transitions }
    }

    fn stride(&self) -> usize {
        self.stack.width + self.groups.len()
    }

    fn ratio(&self, group: &GpGroup, row: &[Fp], r: usize) -> Fp {
        let k = group.wired_cols.len();
        let (b, gm) = (group.beta, group.gamma);
        let mut num = Fp::ONE;
        let mut den = Fp::ONE;
        for (j, &col) in group.wired_cols.iter().enumerate() {
            let v = row[col];
            let id = r * k + j;
            num = num * (v + b * Fp::from_u64(id as u64) + gm);
            den = den * (v + b * Fp::from_u64(group.sigma[id] as u64) + gm);
        }
        num * den.inv()
    }

    /// The witness: regions in the low columns, each group's running product in one
    /// column above them.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let stride = self.stride();
        let total = 1usize << self.log_trace_len();
        let mut trace = fusion::place_traces(&self.stack, &self.regions, stride, total, traces);
        let span = self.stack.span();
        for (g, group) in self.groups.iter().enumerate() {
            let z_col = self.stack.width + g;
            let mut z = Fp::ONE;
            for r in 0..total {
                trace[r * stride + z_col] = z;
                if r < span {
                    let base = r * stride;
                    z = z * self.ratio(group, &trace[base..base + stride], r);
                }
            }
        }
        trace
    }

    fn group_product<F: Felt>(&self, g: usize, group: &GpGroup, window: &[F], periodic: &[F]) -> F {
        let b = F::from_base(group.beta);
        let gm = F::from_base(group.gamma);
        let stride = self.stride();
        let width = self.stack.width;
        let z = window[width + g];
        let z_next = window[stride + width + g];
        let slot = self.group_slots[g];
        let mut num = F::ONE;
        let mut den = F::ONE;
        for (j, &col) in group.wired_cols.iter().enumerate() {
            let v = window[col];
            let id = periodic[slot + 2 * j];
            let sig = periodic[slot + 2 * j + 1];
            num = num * (v + b * id + gm);
            den = den * (v + b * sig + gm);
        }
        let gp_sel = periodic[slot + 2 * group.wired_cols.len()];
        let product = z_next * den - z * num;
        let carry = z_next - z;
        gp_sel * product + (F::ONE - gp_sel) * carry
    }

    fn append_groups<F: Felt>(&self, out: &mut [F], window: &[F], periodic: &[F]) {
        for (g, group) in self.groups.iter().enumerate() {
            out[self.region_transitions + g] = self.group_product(g, group, window, periodic);
        }
    }
}

impl AirExt for WiredMultiExt {
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
        self.append_groups(&mut out, window, periodic);
        out
    }
}

impl Air for WiredMultiExt {
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
        let max_group = self.groups.iter().map(|g| g.wired_cols.len()).max().unwrap_or(0);
        (d + 2).max(max_group + 2)
    }

    fn num_transition(&self) -> usize {
        self.region_transitions + self.groups.len()
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        let span = self.stack.span();
        let mut cols = fusion::base_periodic(&self.stack, &self.regions, total);
        for group in &self.groups {
            let k = group.wired_cols.len();
            for j in 0..k {
                let mut id = alloc::vec![Fp::ZERO; total];
                let mut sig = alloc::vec![Fp::ZERO; total];
                for r in 0..span {
                    id[r] = Fp::from_u64((r * k + j) as u64);
                    sig[r] = Fp::from_u64(group.sigma[r * k + j] as u64);
                }
                cols.push(id);
                cols.push(sig);
            }
            let mut gp_sel = alloc::vec![Fp::ZERO; total];
            for item in gp_sel.iter_mut().take(span) {
                *item = Fp::ONE;
            }
            cols.push(gp_sel);
        }
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
        self.append_groups(&mut out, window, periodic);
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = fusion::base_boundary(&self.stack, &self.regions);
        let span = self.stack.span();
        for g in 0..self.groups.len() {
            b.push((self.stack.width + g, 0, Fp::ONE));
            b.push((self.stack.width + g, span, Fp::ONE));
        }
        b
    }
}
