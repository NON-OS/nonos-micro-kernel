// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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
