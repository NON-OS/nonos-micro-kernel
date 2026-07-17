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

//! The two hashes the transparent proof stack needs: keccak256 for the
//! Fiat-Shamir transcript and the Merkle commitment, and blake3 for the
//! image measurement. Both are carried inside this crate so the prover and
//! the verifier compute identical digests, whichever binary links it.

mod constants;
mod keccak;

use keccak::Keccak;

/// Ethereum-style Keccak-256 (0x01 padding), the transcript and Merkle hash.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::new(512, 32, 0x01);
    hasher.update(data);
    let out = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&out);
    hash
}

/// BLAKE3, the image measurement hash. Matches the bootloader's kernel measure.
pub fn blake3_hash(data: &[u8]) -> [u8; 32] {
    *blake3::hash(data).as_bytes()
}
