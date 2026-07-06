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

use crate::crypto::stark::air::{stark_prove, stark_verify, Fibonacci, Squaring};
use crate::crypto::stark::field::Fp;

extern crate alloc;
use alloc::vec::Vec;

// The end-to-end STARK: a real proof that a computation ran correctly, with no
// trusted setup and hash-only cryptography. The engine is generic over the AIR,
// so the same prover and verifier handle two structurally different problems, a
// squaring chain and a Fibonacci recurrence. These checks exercise the whole
// pipeline (interpolation, low-degree extension, constraint composition, FRI,
// and the trace consistency binding) on the real code, honest and dishonest.

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

fn fibonacci_trace(log_t: u32) -> Vec<Fp> {
    let t = 1usize << log_t;
    let mut trace = Vec::with_capacity(t);
    let (mut a, mut b) = (Fp::ONE, Fp::ONE);
    for _ in 0..t {
        trace.push(a);
        let next = a + b;
        a = b;
        b = next;
    }
    trace
}

#[test]
fn an_honest_squaring_execution_verifies() {
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t, seed };
    let proof = stark_prove(&air, &squaring_trace(log_t, seed), log_blowup, queries);
    assert!(stark_verify(&air, &proof, log_blowup, queries), "honest squaring rejected");
}

#[test]
fn an_honest_fibonacci_execution_verifies() {
    // The same engine, a different computation. This is the generality check.
    let (log_t, log_blowup, queries) = (4u32, 2u32, 32usize);
    let air = Fibonacci { log_t };
    let proof = stark_prove(&air, &fibonacci_trace(log_t), log_blowup, queries);
    assert!(stark_verify(&air, &proof, log_blowup, queries), "honest fibonacci rejected");
}

#[test]
fn honest_executions_prove_across_sizes() {
    for log_t in [2u32, 3, 4] {
        let seed = Fp::from_u64(2 + log_t as u64);
        let air = Squaring { log_t, seed };
        let proof = stark_prove(&air, &squaring_trace(log_t, seed), 3, 32);
        assert!(stark_verify(&air, &proof, 3, 32), "squaring at log_t {log_t} rejected");
    }
}

#[test]
fn a_corrupted_squaring_transition_is_rejected() {
    // Break one step of the chain. The composition is no longer low degree, so
    // FRI rejects the proof honestly built from the bad trace.
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t, seed };
    let mut trace = squaring_trace(log_t, seed);
    trace[4] = trace[4] + Fp::ONE;
    let proof = stark_prove(&air, &trace, log_blowup, queries);
    assert!(!stark_verify(&air, &proof, log_blowup, queries), "a corrupted squaring verified");
}

#[test]
fn a_corrupted_fibonacci_transition_is_rejected() {
    let (log_t, log_blowup, queries) = (4u32, 2u32, 32usize);
    let air = Fibonacci { log_t };
    let mut trace = fibonacci_trace(log_t);
    trace[5] = trace[5] + Fp::ONE;
    let proof = stark_prove(&air, &trace, log_blowup, queries);
    assert!(!stark_verify(&air, &proof, log_blowup, queries), "a corrupted fibonacci verified");
}

#[test]
fn a_wrong_boundary_seed_is_rejected() {
    // The proof is honest, but the verifier is told a different public seed. The
    // boundary quotient it recomputes no longer matches the committed composition.
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let proof =
        stark_prove(&Squaring { log_t, seed }, &squaring_trace(log_t, seed), log_blowup, queries);
    let wrong = Squaring { log_t, seed: Fp::from_u64(4) };
    assert!(!stark_verify(&wrong, &proof, log_blowup, queries), "a wrong boundary seed verified");
}

#[test]
fn a_tampered_trace_opening_is_rejected() {
    // Corrupt one opened trace value. Its Merkle path no longer recomputes the
    // trace root.
    let (log_t, log_blowup, queries) = (3u32, 2u32, 32usize);
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t, seed };
    let mut proof = stark_prove(&air, &squaring_trace(log_t, seed), log_blowup, queries);
    proof.queries[0].window[0] = proof.queries[0].window[0] + Fp::ONE;
    assert!(!stark_verify(&air, &proof, log_blowup, queries), "a tampered trace opening verified");
}
