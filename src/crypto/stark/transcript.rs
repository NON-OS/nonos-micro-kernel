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

//! A Fiat-Shamir transcript over BLAKE3. Each absorb folds its input into a
//! running 32-byte state under a domain tag; each challenge folds a tag in and
//! reads the fresh state. A prover and verifier that absorb the same sequence
//! draw the same challenges, which is what turns the interactive FRI protocol
//! into a non-interactive one, sound in the random-oracle model.

use super::field::Fp;
use crate::crypto::hash::blake3_hash;
use alloc::vec::Vec;

pub struct Transcript {
    state: [u8; 32],
}

impl Transcript {
    pub fn new(label: &[u8]) -> Transcript {
        Transcript { state: blake3_hash(label) }
    }

    fn mix(&mut self, tag: u8, data: &[u8]) {
        let mut buf = Vec::with_capacity(1 + 32 + data.len());
        buf.push(tag);
        buf.extend_from_slice(&self.state);
        buf.extend_from_slice(data);
        self.state = blake3_hash(&buf);
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

    /// Draw a field-element challenge.
    pub fn challenge_fp(&mut self) -> Fp {
        Fp::from_u64(self.squeeze_u64(0x03))
    }

    /// Draw a query index in `[0, bound)`. `bound` is a power of two in FRI, so
    /// masking is unbiased.
    pub fn challenge_index(&mut self, bound: usize) -> usize {
        (self.squeeze_u64(0x04) as usize) & (bound - 1)
    }
}
