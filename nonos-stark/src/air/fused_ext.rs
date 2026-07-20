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

//! The money-grade fusion of several `AirExt` regions into one trace, so a
//! compound statement proves as a single STARK at ~2^-128. The stacking, periodic,
//! boundary, and selected-transition logic live in `fusion`; this is the thin
//! `AirExt` over it. The base `Fused` remains for the recursive verifier.

use super::super::field::{Fp, Fp2};
use super::fusion::{self, Stack};
use super::spec::{Air, AirExt};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct FusedExt {
    regions: Vec<Box<dyn AirExt>>,
    stack: Stack,
}

impl FusedExt {
    pub fn new(regions: Vec<Box<dyn AirExt>>) -> FusedExt {
        let stack = Stack::of(&regions);
        FusedExt { regions, stack }
    }

    /// The fused witness: each region's trace at its offset.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let total = 1usize << self.log_trace_len();
        fusion::place_traces(&self.stack, &self.regions, self.stack.width, total, traces)
    }
}

impl Air for FusedExt {
    fn log_trace_len(&self) -> u32 {
        self.stack.log_span
    }

    fn trace_width(&self) -> usize {
        self.stack.width
    }

    fn window_size(&self) -> usize {
        self.stack.window
    }

    fn constraint_degree(&self) -> usize {
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
        fusion::base_periodic(&self.stack, &self.regions, 1usize << self.log_trace_len())
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stack.width,
            window,
            periodic,
            |i, l, p| self.regions[i].transition(l, p),
        )
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        fusion::base_boundary(&self.stack, &self.regions)
    }
}

impl AirExt for FusedExt {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stack.width,
            window,
            periodic,
            |i, l, p| self.regions[i].transition_ext(l, p),
        )
    }
}
