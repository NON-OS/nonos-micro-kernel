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
use super::field::{Fp, Fp2};

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

    /// Draw a challenge from the degree-2 extension. Money-grade fold and DEEP
    /// challenges are drawn here, not from the base field: the low-degree test's
    /// soundness error is `degree / |challenge field|`, so `Fp2` (~2^128) reaches
    /// `2^-128` where the base field caps near `2^-64`. This is the algebraic
    /// counterpart of the keccak transcript's `challenge_fp2`, so a Poseidon-
    /// committed money-grade proof can have its challenges re-derived in a STARK.
    pub fn challenge_fp2(&mut self) -> Fp2 {
        let c0 = self.challenge();
        let c1 = self.challenge();
        Fp2::new(c0, c1)
    }

    /// The grinding word for a nonce against the current state: the first lane of
    /// the permutation with the nonce injected, not bound in until the winning
    /// nonce is committed, so it cannot be re-searched per query.
    fn pow_word(&self, nonce: u64) -> u64 {
        let mut s = self.state;
        s[0] = s[0] + Fp::from_u64(nonce);
        self.hasher.permute(s)[0].value()
    }

    /// Prover-side grinding: find a nonce whose grinding word has at least `bits`
    /// leading zero bits, then bind it, adding `bits` of proof-of-work.
    pub fn grind(&mut self, bits: u32) -> u64 {
        let mut nonce = 0u64;
        while self.pow_word(nonce).leading_zeros() < bits {
            nonce = nonce.wrapping_add(1);
        }
        self.absorb(Fp::from_u64(nonce));
        nonce
    }

    /// Verifier-side grinding check: accept only if the nonce meets the proof-of-
    /// work, and bind it exactly as the prover did so both draw the same challenges.
    pub fn verify_pow(&mut self, nonce: u64, bits: u32) -> bool {
        if self.pow_word(nonce).leading_zeros() < bits {
            return false;
        }
        self.absorb(Fp::from_u64(nonce));
        true
    }
}
