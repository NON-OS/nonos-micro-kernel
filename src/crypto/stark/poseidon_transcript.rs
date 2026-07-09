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

//! A Fiat-Shamir transcript over the Poseidon permutation, the algebraic
//! counterpart of the BLAKE3 transcript. Absorbing adds a value into the first
//! lane and permutes; a challenge reads the first lane and permutes. It is the
//! same duplex the `FiatShamir` AIR proves, so a proof made with this transcript
//! can have its challenges re-derived inside a STARK: the requirement for
//! recursion.

use super::air::{Poseidon, RATE, WIDTH};
use super::field::Fp;

pub struct PoseidonTranscript {
    hasher: Poseidon,
    state: [Fp; WIDTH],
}

impl PoseidonTranscript {
    pub fn new(hasher: Poseidon) -> PoseidonTranscript {
        PoseidonTranscript { hasher, state: [Fp::ZERO; WIDTH] }
    }

    /// Absorb one field element.
    pub fn absorb(&mut self, value: Fp) {
        self.state[0] = self.state[0] + value;
        self.state = self.hasher.permute(self.state);
    }

    /// Absorb a rate-sized digest, lane by lane.
    pub fn absorb_digest(&mut self, digest: &[Fp; RATE]) {
        for v in digest.iter() {
            self.absorb(*v);
        }
    }

    /// Draw a field-element challenge.
    pub fn challenge(&mut self) -> Fp {
        let c = self.state[0];
        self.state = self.hasher.permute(self.state);
        c
    }

    /// Draw a query index in `[0, bound)`. `bound` is a power of two, so masking
    /// is unbiased.
    pub fn challenge_index(&mut self, bound: usize) -> usize {
        (self.challenge().value() as usize) & (bound - 1)
    }
}
