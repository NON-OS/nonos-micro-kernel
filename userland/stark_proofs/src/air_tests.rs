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

use crate::crypto::stark::air::{stark_prove, stark_verify};
use crate::crypto::stark::field::Fp;

extern crate alloc;
use alloc::vec::Vec;

// The end-to-end STARK: a real proof that a squaring chain t[i+1] = t[i]^2 was
// computed correctly, with no trusted setup and hash-only cryptography. These
// checks exercise the whole pipeline (interpolation, low-degree extension,
// constraint composition, FRI, and the trace consistency binding) on the real
// prover and verifier, for both honest and dishonest traces.

/// The honest squaring trace of length `2^log_t` from `seed`.
fn squaring_trace(log_t: u32, seed: Fp) -> Vec<Fp> {
    let t = 1usize << log_t;
    let mut trace = Vec::with_capacity(t);
    let mut cur = seed;
    for _ in 0..t {
        trace.push(cur);
        cur = cur * cur;
    }
    trace
}

#[test]
fn an_honest_execution_proves_and_verifies() {
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let trace = squaring_trace(log_t, seed);
    let proof = stark_prove(&trace, log_blowup, queries);
    assert!(stark_verify(&proof, seed, log_t, log_blowup, queries), "honest execution rejected");
}

#[test]
fn honest_executions_prove_across_sizes() {
    for (log_t, log_blowup) in [(2u32, 2u32), (3, 2), (4, 3)] {
        let seed = Fp::from_u64(2 + log_t as u64);
        let trace = squaring_trace(log_t, seed);
        let proof = stark_prove(&trace, log_blowup, 32);
        assert!(
            stark_verify(&proof, seed, log_t, log_blowup, 32),
            "honest execution at log_t {log_t} rejected"
        );
    }
}

#[test]
fn a_corrupted_transition_is_rejected() {
    // Break one step of the chain. The composition is no longer low degree, so
    // FRI rejects the proof the prover honestly builds from the bad trace.
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let mut trace = squaring_trace(log_t, seed);
    trace[4] = trace[4] + Fp::ONE;
    let proof = stark_prove(&trace, log_blowup, queries);
    assert!(!stark_verify(&proof, seed, log_t, log_blowup, queries), "a corrupted trace verified");
}

#[test]
fn a_wrong_boundary_seed_is_rejected() {
    // The proof is honest, but the verifier is told a different public seed. The
    // boundary quotient it recomputes no longer matches the committed composition.
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let trace = squaring_trace(log_t, seed);
    let proof = stark_prove(&trace, log_blowup, queries);
    let wrong_seed = Fp::from_u64(4);
    assert!(
        !stark_verify(&proof, wrong_seed, log_t, log_blowup, queries),
        "a wrong boundary seed verified"
    );
}

#[test]
fn a_tampered_trace_opening_is_rejected() {
    // Corrupt one opened trace value. Its Merkle path no longer recomputes the
    // trace root.
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let trace = squaring_trace(log_t, seed);
    let mut proof = stark_prove(&trace, log_blowup, queries);
    proof.queries[0].t_x = proof.queries[0].t_x + Fp::ONE;
    assert!(
        !stark_verify(&proof, seed, log_t, log_blowup, queries),
        "a tampered trace opening verified"
    );
}
