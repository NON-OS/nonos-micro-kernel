// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use blake3;

/// Fiat-Shamir transcript using BLAKE3
#[derive(Clone)]
pub struct Transcript {
    h: blake3::Hasher,
}

impl Transcript {
    /// Create a new transcript with domain separation
    pub fn new(domain: &str) -> Self {
        let mut h = blake3::Hasher::new_derive_key(domain);
        h.update(domain.as_bytes());
        Transcript { h }
    }

    /// Absorb labeled data into the transcript
    pub fn absorb(&mut self, label: &str, data: &[u8]) {
        // Include label length and bytes
        self.h.update(&(label.len() as u32).to_le_bytes());
        self.h.update(label.as_bytes());
        // Include data length and bytes
        self.h.update(&(data.len() as u32).to_le_bytes());
        self.h.update(data);
    }

    /// Absorb a 32-byte value with label
    pub fn absorb32(&mut self, label: &str, value: &[u8; 32]) {
        self.absorb(label, value);
    }

    /// Absorb a u64 value with label
    pub fn absorb_u64(&mut self, label: &str, value: u64) {
        self.absorb(label, &value.to_le_bytes());
    }

    /// Generate a 32-byte challenge
    pub fn challenge32(&self, label: &str) -> [u8; 32] {
        let mut h2 = self.h.clone();
        h2.update(&(label.len() as u32).to_le_bytes());
        h2.update(label.as_bytes());
        *h2.finalize().as_bytes()
    }

    /// Generate a challenge and absorb it back into the transcript
    pub fn challenge32_and_absorb(&mut self, label: &str) -> [u8; 32] {
        let challenge = self.challenge32(label);
        self.absorb(label, &challenge);
        challenge
    }

    /// Get current state hash (for debugging/logging)
    pub fn state_hash(&self) -> [u8; 32] {
        *self.h.clone().finalize().as_bytes()
    }
}

/// Domain separator for boot attestation transcript
pub const TRANSCRIPT_DOMAIN_BOOT: &str = "NONOS:BOOT:ATTESTATION:v1";

/// Domain separator for circuit verification transcript
pub const TRANSCRIPT_DOMAIN_CIRCUIT: &str = "NONOS:CIRCUIT:VERIFY:v1";
