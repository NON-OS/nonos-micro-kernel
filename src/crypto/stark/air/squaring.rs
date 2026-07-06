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

//! The squaring AIR: a chain `t[i+1] = t[i]^2` from a public seed.

use super::super::field::Fp;
use super::spec::Air;
use alloc::vec;
use alloc::vec::Vec;

pub struct Squaring {
    pub log_t: u32,
    pub seed: Fp,
}

impl Air for Squaring {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn window_size(&self) -> usize {
        2
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn transition(&self, window: &[Fp]) -> Vec<Fp> {
        // f(g*x) - f(x)^2
        vec![window[1] - window[0] * window[0]]
    }

    fn boundary(&self) -> Vec<(usize, Fp)> {
        vec![(0, self.seed)]
    }
}
