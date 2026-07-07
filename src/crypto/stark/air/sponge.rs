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

//! A sponge hash preimage proof. The state has three lanes: two rate lanes that
//! absorb the input, and one capacity lane initialized to zero, which is the
//! domain separation that makes the sponge a hash. Each round applies a full x^7
//! S-box to every lane, mixes them through the invertible matrix with rows
//! (2,1,1), (1,2,1), (1,1,2) (determinant four, nonzero in this field), and adds
//! round constants. The proof shows that a public digest, the final rate lanes,
//! is the sponge of some input the proof never reveals: knowledge of a preimage.
//! The round count and constants here are a demonstration instantiation, not a
//! security-reviewed parameter set.

use super::super::field::Fp;
use super::spec::Air;
use alloc::vec;
use alloc::vec::Vec;

pub struct SpongePreimage {
    /// Log2 of the number of permutation rounds.
    pub log_t: u32,
    /// The three round constants, one per lane.
    pub rc: [Fp; 3],
    /// The public digest: the two rate lanes at the final row.
    pub digest: [Fp; 2],
}

impl Air for SpongePreimage {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn trace_width(&self) -> usize {
        3
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        7
    }

    fn num_transition(&self) -> usize {
        3
    }

    fn transition(&self, window: &[Fp]) -> Vec<Fp> {
        // window = [a, b, c, a_next, b_next, c_next].
        let two = Fp::from_u64(2);
        let (sa, sb, sc) = (window[0].pow(7), window[1].pow(7), window[2].pow(7));
        vec![
            window[3] - (two * sa + sb + sc + self.rc[0]),
            window[4] - (sa + two * sb + sc + self.rc[1]),
            window[5] - (sa + sb + two * sc + self.rc[2]),
        ]
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let last = (1usize << self.log_t) - 1;
        vec![
            // Capacity lane starts at zero: the sponge initialization.
            (2, 0, Fp::ZERO),
            // The final rate lanes are the public digest.
            (0, last, self.digest[0]),
            (1, last, self.digest[1]),
        ]
    }
}
