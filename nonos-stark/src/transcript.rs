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

//! A Fiat-Shamir transcript over Keccak256 (the EVM-native hash, so the Each absorb folds its input into a
//! running 32-byte state under a domain tag; each challenge folds a tag in and
//! reads the fresh state. A prover and verifier that absorb the same sequence
//! draw the same challenges, which is what turns the interactive FRI protocol
//! into a non-interactive one, sound in the random-oracle model.

use super::field::{Fp, Fp2};
use crate::hash::keccak256;
use alloc::vec::Vec;

pub struct Transcript {
    state: [u8; 32],
}

impl Transcript {
    pub fn new(label: &[u8]) -> Transcript {
        Transcript { state: keccak256(label) }
    }

    fn mix(&mut self, tag: u8, data: &[u8]) {
        let mut buf = Vec::with_capacity(1 + 32 + data.len());
        buf.push(tag);
        buf.extend_from_slice(&self.state);
        buf.extend_from_slice(data);
        self.state = keccak256(&buf);
    }

    /// Absorb a commitment root.
    pub fn absorb_digest(&mut self, digest: &[u8; 32]) {
        self.mix(0x01, digest);
    }

    /// Absorb a field element.
    pub fn absorb_fp(&mut self, value: Fp) {
        self.mix(0x02, &value.value().to_le_bytes());
    }

    fn squeeze_u64(&mut self, tag: u8) -> u64 {
        self.mix(tag, &[]);
        let s = &self.state;
        u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]])
    }

    /// Draw a base-field challenge. Retained for query index derivation; fold and
    /// DEEP challenges use `challenge_fp2` for soundness (see below).
    pub fn challenge_fp(&mut self) -> Fp {
        Fp::from_u64(self.squeeze_u64(0x03))
    }

    /// Draw a query index in `[0, bound)`. `bound` is a power of two in FRI, so
    /// masking is unbiased.
    pub fn challenge_index(&mut self, bound: usize) -> usize {
        (self.squeeze_u64(0x04) as usize) & (bound - 1)
    }

    /// Draw a challenge from the degree-2 extension `Fp2`. Fold and DEEP challenges
    /// are drawn here, not from the base field: the low-degree test's soundness
    /// error is `degree / |challenge field|`, so the base field caps it near
    /// `2^-64`, while `Fp2` (~`2^128`) reaches `2^-128`. The bound is proved in
    /// `Nonos.Stark.Soundness.the_soundness_error_is_below_the_degree`.
    pub fn challenge_fp2(&mut self) -> Fp2 {
        let c0 = Fp::from_u64(self.squeeze_u64(0x06));
        let c1 = Fp::from_u64(self.squeeze_u64(0x07));
        Fp2::new(c0, c1)
    }

    /// The grinding word for a nonce against the current state, not folded in until
    /// the winning nonce is committed. Keccak256 keeps the search a random oracle.
    fn pow_word(&self, nonce: u64) -> u64 {
        let mut buf = Vec::with_capacity(1 + 32 + 8);
        buf.push(0x05);
        buf.extend_from_slice(&self.state);
        buf.extend_from_slice(&nonce.to_le_bytes());
        let h = keccak256(&buf);
        u64::from_le_bytes([h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]])
    }

    /// Prover-side grinding: search for a nonce whose grinding word has at least
    /// `bits` leading zero bits, then bind it into the transcript. This adds `bits`
    /// of proof-of-work to the query soundness, raising the cost of resampling
    /// challenges to forge a proof.
    pub fn grind(&mut self, bits: u32) -> u64 {
        let nonce = self.grind_search(bits);
        self.mix(0x05, &nonce.to_le_bytes());
        nonce
    }

    /// The proof-of-work search. Serially it is the smallest nonce meeting the
    /// work; over `bits` in the 30s that is billions of hashes on one core, the
    /// dominant cost of a deployment emit. The parallel form searches the nonce
    /// space in blocks across every core and returns the first (lowest) hit in
    /// each block, so it finds the identical smallest nonce as the serial search
    /// — bit-exact, just spread across cores.
    #[cfg(not(feature = "parallel"))]
    fn grind_search(&self, bits: u32) -> u64 {
        let mut nonce = 0u64;
        while self.pow_word(nonce).leading_zeros() < bits {
            nonce = nonce.wrapping_add(1);
        }
        nonce
    }

    #[cfg(feature = "parallel")]
    fn grind_search(&self, bits: u32) -> u64 {
        use rayon::prelude::*;
        const BLOCK: u64 = 1 << 26;
        let mut base = 0u64;
        loop {
            let hit = (base..base + BLOCK)
                .into_par_iter()
                .find_first(|&n| self.pow_word(n).leading_zeros() >= bits);
            if let Some(n) = hit {
                return n;
            }
            base += BLOCK;
        }
    }

    /// Verifier-side grinding check: accept only if the nonce meets the `bits`
    /// proof-of-work, and on success bind it in exactly as the prover did so both
    /// draw the same subsequent challenges.
    pub fn verify_pow(&mut self, nonce: u64, bits: u32) -> bool {
        if self.pow_word(nonce).leading_zeros() < bits {
            return false;
        }
        self.mix(0x05, &nonce.to_le_bytes());
        true
    }
}
