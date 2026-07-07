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
    stark_prove, stark_verify, Fibonacci, Permutation2, PowerChain, SpongePreimage, Squaring,
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

/// Run the width-three sponge on a secret input and return the trace and the
/// public digest (the final rate lanes). The capacity lane starts at zero.
fn sponge_trace(log_t: u32, in0: Fp, in1: Fp, rc: [Fp; 3]) -> (Vec<Fp>, [Fp; 2]) {
    let t = 1usize << log_t;
    let two = Fp::from_u64(2);
    let mut trace = Vec::with_capacity(3 * t);
    let (mut a, mut b, mut c) = (in0, in1, Fp::ZERO);
    for _ in 0..t {
        trace.push(a);
        trace.push(b);
        trace.push(c);
        let (sa, sb, sc) = (a.pow(7), b.pow(7), c.pow(7));
        let na = two * sa + sb + sc + rc[0];
        let nb = sa + two * sb + sc + rc[1];
        let nc = sa + sb + two * sc + rc[2];
        a = na;
        b = nb;
        c = nc;
    }
    let digest = [trace[(t - 1) * 3], trace[(t - 1) * 3 + 1]];
    (trace, digest)
}

#[test]
fn an_honest_hash_preimage_verifies() {
    // Prove knowledge of an input that sponges to a public digest, without the
    // proof carrying the input in its public statement.
    let rc = [Fp::from_u64(7), Fp::from_u64(11), Fp::from_u64(13)];
    let (trace, digest) = sponge_trace(4, Fp::from_u64(42), Fp::from_u64(99), rc);
    let air = SpongePreimage { log_t: 4, rc, digest };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "honest preimage rejected");
}

#[test]
fn a_wrong_hash_digest_is_rejected() {
    let rc = [Fp::from_u64(7), Fp::from_u64(11), Fp::from_u64(13)];
    let (trace, digest) = sponge_trace(4, Fp::from_u64(42), Fp::from_u64(99), rc);
    let air = SpongePreimage { log_t: 4, rc, digest: [digest[0] + Fp::ONE, digest[1]] };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a wrong digest verified");
}

#[test]
fn a_corrupted_hash_round_is_rejected() {
    let rc = [Fp::from_u64(7), Fp::from_u64(11), Fp::from_u64(13)];
    let (mut trace, digest) = sponge_trace(4, Fp::from_u64(42), Fp::from_u64(99), rc);
    trace[12] = trace[12] + Fp::ONE;
    let air = SpongePreimage { log_t: 4, rc, digest };
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a corrupted hash round verified");
}

#[test]
fn a_nonzero_capacity_initialization_is_rejected() {
    // A prover that seeds the capacity lane with anything but zero is computing a
    // different function; the sponge-initialization boundary rejects it.
    let rc = [Fp::from_u64(7), Fp::from_u64(11), Fp::from_u64(13)];
    let t = 1usize << 4;
    let two = Fp::from_u64(2);
    let mut trace = Vec::with_capacity(3 * t);
    let (mut a, mut b, mut c) = (Fp::from_u64(42), Fp::from_u64(99), Fp::from_u64(5));
    for _ in 0..t {
        trace.push(a);
        trace.push(b);
        trace.push(c);
        let (sa, sb, sc) = (a.pow(7), b.pow(7), c.pow(7));
        a = two * sa + sb + sc + rc[0];
        b = sa + two * sb + sc + rc[1];
        c = sa + sb + two * sc + rc[2];
    }
    let digest = [trace[(t - 1) * 3], trace[(t - 1) * 3 + 1]];
    let air = SpongePreimage { log_t: 4, rc, digest };
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
