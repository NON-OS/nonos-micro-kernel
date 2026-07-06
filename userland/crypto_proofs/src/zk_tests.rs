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

use crate::crypto::zk_kernel::{KernelZkVerifier, ZkResult};

extern crate alloc;
use alloc::vec::Vec;

// The kernel ZK verifier consumes attacker-controlled proof bytes. Two
// properties over a large adversarial corpus: it never panics, and it never
// accepts a random proof as valid (soundness sanity: a forgery would have to
// break the underlying cryptography, not just hit a decode path).

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

fn proof_bytes(len: usize, seed: u32) -> Vec<u8> {
    let mut s = seed | 1;
    (0..len).map(|_| (xorshift(&mut s) & 0xff) as u8).collect()
}

#[test]
fn verify_plonk_never_panics_and_never_accepts_garbage() {
    let mut v = KernelZkVerifier::new();
    for &len in &[0usize, 1, 8, 32, 33, 64, 128, 256, 512, 1024] {
        for seed in 0..3_000u32 {
            let pb = proof_bytes(len, seed);
            // Vary the number of public inputs (0..3), each adversarial.
            let n = (seed as usize) % 4;
            let pis: Vec<[u8; 32]> = (0..n)
                .map(|i| {
                    let mut a = [0u8; 32];
                    a.copy_from_slice(&proof_bytes(32, seed ^ (i as u32 + 1)));
                    a
                })
                .collect();
            assert_ne!(
                v.verify_plonk(&pb, &pis),
                ZkResult::Valid,
                "a random PLONK proof must never validate"
            );
        }
    }
}

#[test]
fn verify_range_never_panics_and_never_accepts_garbage() {
    let mut v = KernelZkVerifier::new();
    for &len in &[0usize, 1, 8, 32, 33, 64, 128, 256, 512, 1024] {
        for seed in 0..4_000u32 {
            assert_ne!(
                v.verify_range(&proof_bytes(len, seed)),
                ZkResult::Valid,
                "a random range proof must never validate"
            );
        }
    }
}

#[test]
fn malformed_short_inputs_are_reported_not_crashed() {
    let mut v = KernelZkVerifier::new();
    // Range proofs shorter than the minimum framing are rejected as malformed.
    for len in 0..33usize {
        assert_eq!(v.verify_range(&proof_bytes(len, len as u32)), ZkResult::MalformedProof);
    }
    // Empty PLONK input decodes to a malformed proof, never a panic or Valid.
    assert_ne!(v.verify_plonk(&[], &[]), ZkResult::Valid);
}
