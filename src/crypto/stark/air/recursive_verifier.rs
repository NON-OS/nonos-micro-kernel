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

//! The recursive FRI verifier, assembled in steps. This file holds the parts
//! that do not depend on the interactive challenge transcript or the per-query
//! verifier, so they stand alone and are proven here; the transcript-driven
//! challenge derivation and the per-query membership-and-fold checks compose in
//! as those gadgets land.
//!
//! First component: the low-degree conclusion. FRI folds a codeword down to a
//! final layer that an honest prover can only reach if the original was low
//! degree, and that final layer must be a single repeated constant. This AIR
//! proves exactly that in-circuit: the witnessed final layer is constant and
//! equals the value the proof committed. It is the last check the recursive
//! verifier makes, and the one gate a high-degree inner proof cannot pass.

use super::super::field::Fp;
use super::spec::Air;
use alloc::vec;
use alloc::vec::Vec;

/// Proof that a FRI final layer is the constant it committed to.
pub struct FinalLayerConstant {
    /// Log2 of the padded final-layer length.
    log_len: u32,
    /// The committed final-layer value the whole layer must equal.
    value: Fp,
}

impl FinalLayerConstant {
    pub fn new(log_len: u32, value: Fp) -> FinalLayerConstant {
        FinalLayerConstant { log_len, value }
    }

    /// The honest trace: the committed value repeated across the padded layer.
    pub fn trace(&self) -> Vec<Fp> {
        vec![self.value; 1usize << self.log_len]
    }
}

impl Air for FinalLayerConstant {
    fn log_trace_len(&self) -> u32 {
        self.log_len
    }

    fn trace_width(&self) -> usize {
        1
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        1
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn transition(&self, window: &[Fp], _periodic: &[Fp]) -> Vec<Fp> {
        // Each value equals its successor: the layer is constant.
        vec![window[1] - window[0]]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // That constant is the committed final-layer value.
        vec![(0, 0, self.value)]
    }
}
