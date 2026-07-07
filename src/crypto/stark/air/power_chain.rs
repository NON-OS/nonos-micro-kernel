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

//! An iterated S-box chain: `t[i+1] = t[i]^7 + c`. Raising to the seventh power
//! is a permutation of this field (7 is coprime to the group order), the same
//! low-degree S-box the Goldilocks hash permutations use. Proving a chain of it
//! is a computational-integrity proof of a hash-style, verifiable-delay
//! computation: the prover shows a public output is the result of applying the
//! permutation `T` times to some starting value, without revealing that value.
//! The degree-7 transition is what exercises the engine's high-degree path.

use super::super::field::Fp;
use super::spec::Air;
use alloc::vec;
use alloc::vec::Vec;

pub struct PowerChain {
    pub log_t: u32,
    /// The round constant added after each S-box.
    pub c: Fp,
    /// The public final value the chain must reach.
    pub output: Fp,
}

impl Air for PowerChain {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn trace_width(&self) -> usize {
        1
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        7
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn transition(&self, window: &[Fp], _periodic: &[Fp]) -> Vec<Fp> {
        // f(g*x) - (f(x)^7 + c)
        let x = window[0];
        let x7 = x.pow(7);
        vec![window[1] - (x7 + self.c)]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // column 0, last row, the public output.
        vec![(0, (1usize << self.log_t) - 1, self.output)]
    }
}
