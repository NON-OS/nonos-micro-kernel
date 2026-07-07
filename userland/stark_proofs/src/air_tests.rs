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

use crate::crypto::stark::air::{
    stark_prove, stark_verify, Fibonacci, Permutation2, Poseidon, PowerChain, Squaring, RATE, WIDTH,
};
use crate::crypto::stark::field::Fp;

extern crate alloc;
use alloc::vec::Vec;

// The end-to-end STARK: a real proof that a computation ran correctly, with no
// trusted setup and hash-only cryptography. The engine is generic over the AIR,
// so one prover and verifier handle three structurally different problems: a
// squaring chain, a Fibonacci recurrence, and an iterated x^7 S-box chain (the
// hash-style, high-degree case). The evaluation domain is derived from each
// AIR's constraint degree. These checks run the whole pipeline on real code,
// honest and dishonest.

const QUERIES: usize = 32;

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

/// The honest S-box chain t[i+1] = t[i]^7 + c from a starting value, and its
/// public final output.
fn power_chain_trace(log_t: u32, start: Fp, c: Fp) -> (Vec<Fp>, Fp) {
    let t = 1usize << log_t;
    let mut trace = Vec::with_capacity(t);
    let mut cur = start;
    for _ in 0..t {
        trace.push(cur);
        cur = cur.pow(7) + c;
    }
    let output = trace[t - 1];
    (trace, output)
}

#[test]
fn an_honest_squaring_execution_verifies() {
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 3, seed };
    let proof = stark_prove(&air, &squaring_trace(3, seed), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "honest squaring rejected");
}

#[test]
fn an_honest_fibonacci_execution_verifies() {
    let air = Fibonacci { log_t: 4 };
    let proof = stark_prove(&air, &fibonacci_trace(4), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "honest fibonacci rejected");
}

#[test]
fn an_honest_sbox_chain_verifies() {
    // The hash-style, degree-7 case: prove a public output is the result of
    // applying the x^7 permutation T times to a starting value.
    let (c, start) = (Fp::from_u64(11), Fp::from_u64(2));
    let (trace, output) = power_chain_trace(4, start, c);
    let air = PowerChain { log_t: 4, c, output };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "honest s-box chain rejected");
}

/// The honest width-two permutation chain and its public final state.
fn permutation2_trace(log_t: u32, x0: Fp, y0: Fp, rc0: Fp, rc1: Fp) -> (Vec<Fp>, [Fp; 2]) {
    let t = 1usize << log_t;
    let mut trace = Vec::with_capacity(2 * t);
    let (mut x, mut y) = (x0, y0);
    for _ in 0..t {
        trace.push(x);
        trace.push(y);
        let nx = x.pow(7) + y + rc0;
        let ny = x + y.pow(7) + rc1;
        x = nx;
        y = ny;
    }
    let out = [trace[(t - 1) * 2], trace[(t - 1) * 2 + 1]];
    (trace, out)
}

#[test]
fn an_honest_permutation_chain_verifies() {
    // A width-two state under an x^7 permutation round: the multi-column, hash
    // round shaped case. The same engine, a two-element state.
    let (rc0, rc1) = (Fp::from_u64(13), Fp::from_u64(17));
    let (trace, out) = permutation2_trace(4, Fp::from_u64(2), Fp::from_u64(5), rc0, rc1);
    let air = Permutation2 { log_t: 4, rc0, rc1, out };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "honest permutation chain rejected");
}

#[test]
fn a_corrupted_permutation_step_is_rejected() {
    let (rc0, rc1) = (Fp::from_u64(13), Fp::from_u64(17));
    let (mut trace, out) = permutation2_trace(4, Fp::from_u64(2), Fp::from_u64(5), rc0, rc1);
    // Corrupt the y column at some row.
    trace[9] = trace[9] + Fp::ONE;
    let air = Permutation2 { log_t: 4, rc0, rc1, out };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a corrupted permutation step verified");
}

#[test]
fn a_wrong_permutation_output_is_rejected() {
    let (rc0, rc1) = (Fp::from_u64(13), Fp::from_u64(17));
    let (trace, out) = permutation2_trace(4, Fp::from_u64(2), Fp::from_u64(5), rc0, rc1);
    let air = Permutation2 { log_t: 4, rc0, rc1, out: [out[0] + Fp::ONE, out[1]] };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a wrong permutation output verified");
}

/// Number of Poseidon rounds used by the tests, as log2. The round count is a
/// security parameter; this is a representative value that exercises the engine.
const POSEIDON_LOG_T: u32 = 5;

/// Build a Poseidon trace from a full initial state, returning the trace and the
/// digest (the rate lanes at the final row).
fn poseidon_trace(air: &Poseidon, initial: [Fp; WIDTH], log_t: u32) -> (Vec<Fp>, [Fp; RATE]) {
    let t = 1usize << log_t;
    let mut state = initial;
    let mut trace = Vec::with_capacity(WIDTH * t);
    for r in 0..t {
        trace.extend_from_slice(&state);
        if r < t - 1 {
            state = air.round(&state, r);
        }
    }
    let mut digest = [Fp::ZERO; RATE];
    digest.copy_from_slice(&trace[(t - 1) * WIDTH..(t - 1) * WIDTH + RATE]);
    (trace, digest)
}

/// A full initial state that absorbs a rate-sized input with the capacity zeroed.
fn absorb(input: [Fp; RATE]) -> [Fp; WIDTH] {
    let mut state = [Fp::ZERO; WIDTH];
    state[..RATE].copy_from_slice(&input);
    state
}

fn sample_input() -> [Fp; RATE] {
    [Fp::from_u64(11), Fp::from_u64(22), Fp::from_u64(33), Fp::from_u64(44)]
}

#[test]
fn poseidon_hashing_diffuses_and_is_deterministic() {
    // A real hash: one changed input lane changes every digest lane (full
    // diffusion through the MDS layer over the rounds), and it is deterministic.
    let air = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let a = air.hash(&sample_input());
    let mut other = sample_input();
    other[3] = other[3] + Fp::ONE;
    let b = air.hash(&other);
    for i in 0..RATE {
        assert_ne!(a[i], b[i], "digest lane {i} did not diffuse");
    }
    assert_eq!(a, air.hash(&sample_input()), "hash is not deterministic");
}

#[test]
fn an_honest_poseidon_preimage_verifies() {
    // Prove knowledge of an input that Poseidon-hashes to a public digest,
    // without the input appearing in the public statement.
    let params = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let (trace, digest) = poseidon_trace(&params, absorb(sample_input()), POSEIDON_LOG_T);
    let air = Poseidon::new(POSEIDON_LOG_T, digest);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "honest poseidon preimage rejected");
}

#[test]
fn a_wrong_poseidon_digest_is_rejected() {
    let params = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let (trace, digest) = poseidon_trace(&params, absorb(sample_input()), POSEIDON_LOG_T);
    let wrong = [digest[0] + Fp::ONE, digest[1], digest[2], digest[3]];
    let air = Poseidon::new(POSEIDON_LOG_T, wrong);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a wrong poseidon digest verified");
}

#[test]
fn a_corrupted_poseidon_round_is_rejected() {
    let params = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let (mut trace, digest) = poseidon_trace(&params, absorb(sample_input()), POSEIDON_LOG_T);
    // Corrupt one lane of a middle row: the round no longer holds there.
    trace[WIDTH * 3 + 2] = trace[WIDTH * 3 + 2] + Fp::ONE;
    let air = Poseidon::new(POSEIDON_LOG_T, digest);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a corrupted poseidon round verified");
}

#[test]
fn a_nonzero_capacity_initialization_is_rejected() {
    // Seeding the capacity with anything but zero computes a different function;
    // the sponge-initialization boundary rejects it.
    let params = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let mut initial = absorb(sample_input());
    initial[RATE] = Fp::from_u64(5);
    let (trace, digest) = poseidon_trace(&params, initial, POSEIDON_LOG_T);
    let air = Poseidon::new(POSEIDON_LOG_T, digest);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a nonzero capacity init verified");
}

#[test]
fn honest_executions_prove_across_sizes() {
    for log_t in [2u32, 3, 4] {
        let seed = Fp::from_u64(2 + log_t as u64);
        let air = Squaring { log_t, seed };
        let proof = stark_prove(&air, &squaring_trace(log_t, seed), QUERIES);
        assert!(stark_verify(&air, &proof, QUERIES), "squaring at log_t {log_t} rejected");
    }
}

#[test]
fn a_corrupted_squaring_transition_is_rejected() {
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 3, seed };
    let mut trace = squaring_trace(3, seed);
    trace[4] = trace[4] + Fp::ONE;
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a corrupted squaring verified");
}

#[test]
fn a_corrupted_fibonacci_transition_is_rejected() {
    let air = Fibonacci { log_t: 4 };
    let mut trace = fibonacci_trace(4);
    trace[5] = trace[5] + Fp::ONE;
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a corrupted fibonacci verified");
}

#[test]
fn a_corrupted_sbox_step_is_rejected() {
    let (c, start) = (Fp::from_u64(11), Fp::from_u64(2));
    let (mut trace, output) = power_chain_trace(4, start, c);
    trace[6] = trace[6] + Fp::ONE;
    let air = PowerChain { log_t: 4, c, output };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a corrupted s-box step verified");
}

#[test]
fn a_wrong_sbox_output_is_rejected() {
    // The chain is honest, but the claimed public output is wrong.
    let (c, start) = (Fp::from_u64(11), Fp::from_u64(2));
    let (trace, output) = power_chain_trace(4, start, c);
    let air = PowerChain { log_t: 4, c, output: output + Fp::ONE };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a wrong s-box output verified");
}

#[test]
fn a_wrong_boundary_seed_is_rejected() {
    let seed = Fp::from_u64(3);
    let proof = stark_prove(&Squaring { log_t: 3, seed }, &squaring_trace(3, seed), QUERIES);
    let wrong = Squaring { log_t: 3, seed: Fp::from_u64(4) };
    assert!(!stark_verify(&wrong, &proof, QUERIES), "a wrong boundary seed verified");
}

#[test]
fn a_tampered_trace_opening_is_rejected() {
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 3, seed };
    let mut proof = stark_prove(&air, &squaring_trace(3, seed), QUERIES);
    proof.queries[0].window[0] = proof.queries[0].window[0] + Fp::ONE;
    assert!(!stark_verify(&air, &proof, QUERIES), "a tampered trace opening verified");
}
