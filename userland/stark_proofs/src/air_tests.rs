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
    stark_prove, stark_verify, Air, CopyConstraint, FiatShamir, Fibonacci, FriFold, Fused,
    MerkleMembership, MultiMembership, Opening, Permutation, Permutation2, Poseidon, PowerChain,
    Squaring, TraceFold, Wired, RATE, WIDTH,
};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poseidon_merkle::PoseidonMerkleTree;
use alloc::boxed::Box;

extern crate alloc;
use alloc::vec::Vec;

fn xs(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

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
    proof.queries[0].trace[0] = proof.queries[0].trace[0] + Fp::ONE;
    assert!(!stark_verify(&air, &proof, QUERIES), "a tampered trace opening verified");
}

#[test]
fn a_full_scale_poseidon_preimage_verifies() {
    // The scale check: 256 rounds, a width-8 trace, an evaluation domain of
    // 4096 points. This is the NTT prover at work; the quadratic extension it
    // replaced would not finish this in reasonable test time.
    let log_t = 8u32;
    let params = Poseidon::new(log_t, [Fp::ZERO; RATE]);
    let (trace, digest) = poseidon_trace(&params, absorb(sample_input()), log_t);
    let air = Poseidon::new(log_t, digest);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "full-scale poseidon preimage rejected");
}

#[test]
fn a_long_squaring_chain_verifies() {
    // A 1024-row single-column trace, domain 4096.
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 10, seed };
    let proof = stark_prove(&air, &squaring_trace(10, seed), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "long squaring chain rejected");
}

fn inject(node: [Fp; RATE], sibling: [Fp; RATE], right: bool) -> [Fp; WIDTH] {
    let mut state = [Fp::ZERO; WIDTH];
    if !right {
        state[..RATE].copy_from_slice(&node);
        state[RATE..].copy_from_slice(&sibling);
    } else {
        state[..RATE].copy_from_slice(&sibling);
        state[RATE..].copy_from_slice(&node);
    }
    state
}

/// Build the Poseidon-state trace of a Merkle path of any depth: compress the
/// node with each sibling by the index bit, place the root at the checkpoint,
/// and let any padding slots run the permutation freely.
fn membership_trace(
    hasher: &Poseidon,
    leaf: [Fp; RATE],
    siblings: &[[Fp; RATE]],
    directions: &[bool],
    log_rounds: u32,
) -> Vec<Fp> {
    let l = 1usize << log_rounds;
    let depth = siblings.len();
    let slots = (depth + 1).next_power_of_two();
    let n = slots * l;

    let mut rows: Vec<[Fp; WIDTH]> = Vec::with_capacity(n);
    let mut state = inject(leaf, siblings[0], directions[0]);
    for r in 0..n {
        rows.push(state);
        let pr = hasher.round_with_rc(&state, &hasher.round_constant(r % l));
        if r % l == l - 1 && r < depth * l {
            let m = (r + 1) / l;
            let mut node = [Fp::ZERO; RATE];
            node.copy_from_slice(&pr[..RATE]);
            if m < depth {
                state = inject(node, siblings[m], directions[m]);
            } else {
                state = inject(node, [Fp::ZERO; RATE], false);
            }
        } else {
            state = pr;
        }
    }

    let mut trace = Vec::with_capacity(n * WIDTH);
    for row in &rows {
        trace.extend_from_slice(row);
    }
    trace
}

fn merkle_leaves(n: usize) -> Vec<[Fp; RATE]> {
    (0..n)
        .map(|i| {
            let mut d = [Fp::ZERO; RATE];
            for (c, cell) in d.iter_mut().enumerate() {
                *cell = Fp::from_u64((i * RATE + c + 1) as u64);
            }
            d
        })
        .collect()
}

fn prove_membership(
    hasher: &Poseidon,
    leaves: &[[Fp; RATE]],
    index: usize,
    log_rounds: u32,
) -> bool {
    let tree = PoseidonMerkleTree::commit(hasher, leaves);
    let root = tree.root();
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, root, path, directions);
    let proof = stark_prove(&air, &trace, QUERIES);
    stark_verify(&air, &proof, QUERIES)
}

#[test]
fn a_merkle_membership_proof_verifies() {
    // Prove, inside a STARK, that a leaf opens to a public Poseidon Merkle root:
    // the commitment check is now itself a proof, the core recursion step.
    let log_rounds = 3u32; // 8-round hash
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    assert!(prove_membership(&hasher, &merkle_leaves(8), 5, log_rounds), "membership rejected");
}

// The capsule attestation gate, end to end. Enroll a set of capsule measurements
// into a policy root, prove membership bound to the capsule identity, serialize the
// proof, and gate on the kernel's verify_membership_attestation. A proof passes only
// under the identity it was drawn for and only against the enrolled root, so a
// forged identity or a foreign root is refused. This is the attestation a spawn
// gates on: the leaf (the capsule secret) stays private, the path is public.
#[test]
fn the_capsule_attestation_gate_accepts_enrolled_and_rejects_forged() {
    use crate::crypto::stark::air::{
        serialize_proof, stark_prove_bound, verify_membership_attestation,
    };
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);

    // The enrolled capsule measurements, committed to the kernel's policy root.
    let leaves = merkle_leaves(8);
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let root = tree.root();

    // This capsule sits at slot 5; its identity binds the attestation.
    let index = 5usize;
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let context = b"capsule:terminal:v1";

    // Enrollment proves knowledge of the enrolled leaf, bound to the identity.
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air =
        MerkleMembership::new(hasher.clone(), log_rounds, root, path.clone(), directions.clone());
    let proof = stark_prove_bound(&air, &trace, QUERIES, context);
    let bytes = serialize_proof(&proof);

    // The kernel gate accepts the enrolled capsule under its own identity.
    assert!(
        verify_membership_attestation(
            &hasher,
            log_rounds,
            root,
            &path,
            &directions,
            QUERIES,
            &bytes,
            context
        ),
        "an enrolled capsule attestation was rejected"
    );
    // The same proof presented under a different identity is refused.
    assert!(
        !verify_membership_attestation(
            &hasher,
            log_rounds,
            root,
            &path,
            &directions,
            QUERIES,
            &bytes,
            b"capsule:impostor"
        ),
        "an attestation passed under the wrong capsule identity"
    );
    // A foreign policy root is refused.
    let mut bad_root = root;
    bad_root[0] = bad_root[0] + Fp::from_u64(1);
    assert!(
        !verify_membership_attestation(
            &hasher,
            log_rounds,
            bad_root,
            &path,
            &directions,
            QUERIES,
            &bytes,
            context
        ),
        "an attestation passed against a forged policy root"
    );
}

// The production-strength attestation: the same membership gate proven at
// money-grade soundness (extension-field challenges, rate one sixteenth, grinding)
// and bound to the capsule identity. The base gate is a demonstration rate; this is
// the ~128-bit gate a spawn can actually rely on, and it still refuses a foreign
// identity.
#[test]
fn the_capsule_attestation_holds_at_money_grade_soundness() {
    use crate::crypto::stark::air::{stark_prove_ext_blown_bound, stark_verify_ext_blown_bound};
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let root = tree.root();
    let index = 5usize;
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let context = b"capsule:terminal:v1";

    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, root, path, directions);
    // Rate one sixteenth, 32 queries, 16 grinding bits: ~128-bit conjectured.
    let proof = stark_prove_ext_blown_bound(&air, &trace, 32, 16, 3, context);
    assert!(
        stark_verify_ext_blown_bound(&air, &proof, 32, 16, 3, context),
        "the money-grade attestation was rejected under its own identity"
    );
    assert!(
        !stark_verify_ext_blown_bound(&air, &proof, 32, 16, 3, b"capsule:impostor"),
        "the money-grade attestation passed under the wrong identity"
    );
}

// The whole gate on real inputs: measure actual capsule images to Poseidon leaves,
// enroll them into the policy root, and let a capsule attest its own measurement at
// money-grade soundness bound to its identity. A capsule whose image was never
// enrolled cannot attest, because its measurement reaches no path to the root. This
// is what makes the leaves mean something: enrollment is measurement, not an
// arbitrary secret.
#[test]
fn a_measured_capsule_enrolls_and_attests_at_money_grade() {
    use crate::crypto::stark::air::{
        measure_capsule, stark_prove_ext_blown_bound, stark_verify_ext_blown_bound,
    };
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);

    let images: [&[u8]; 4] = [
        b"capsule:terminal image bytes",
        b"capsule:net_core image bytes",
        b"capsule:editor image bytes",
        b"capsule:browser image bytes",
    ];
    let leaves: Vec<[Fp; RATE]> = images.iter().map(|img| measure_capsule(&hasher, img)).collect();
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let root = tree.root();

    let index = 0usize;
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let context = b"capsule:terminal:v1";

    // The enrolled capsule attests its own measurement.
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air =
        MerkleMembership::new(hasher.clone(), log_rounds, root, path.clone(), directions.clone());
    let proof = stark_prove_ext_blown_bound(&air, &trace, 32, 16, 3, context);
    assert!(
        stark_verify_ext_blown_bound(&air, &proof, 32, 16, 3, context),
        "the enrolled measured capsule was rejected"
    );

    // A capsule whose image was never enrolled cannot attest.
    let rogue = measure_capsule(&hasher, b"capsule:rogue never enrolled");
    let rogue_trace = membership_trace(&hasher, rogue, &path, &directions, log_rounds);
    let rogue_proof = stark_prove_ext_blown_bound(&air, &rogue_trace, 32, 16, 3, context);
    assert!(
        !stark_verify_ext_blown_bound(&air, &rogue_proof, 32, 16, 3, context),
        "a rogue capsule measurement attested against the policy root"
    );
}

// The kernel gate's path exactly: a capsule ships a money-grade attestation as
// bytes, the kernel parses it from an untrusted trailer and verifies it bound to the
// capsule identity. The proof survives the round trip, the wrong identity is
// refused, and a truncated trailer parses to nothing rather than panicking.
#[test]
fn a_money_grade_attestation_survives_serialization() {
    use crate::crypto::stark::air::{
        deserialize_proof_ext, measure_capsule, serialize_proof_ext, stark_prove_ext_blown_bound,
        stark_verify_ext_blown_bound,
    };
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let images: [&[u8]; 4] =
        [b"capsule:a bytes", b"capsule:b bytes", b"capsule:c bytes", b"capsule:d bytes"];
    let leaves: Vec<[Fp; RATE]> = images.iter().map(|img| measure_capsule(&hasher, img)).collect();
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let root = tree.root();
    let index = 2usize;
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let context = b"capsule:c:v1";

    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, root, path, directions);
    let proof = stark_prove_ext_blown_bound(&air, &trace, 32, 16, 3, context);

    let bytes = serialize_proof_ext(&proof);
    let parsed = deserialize_proof_ext(&bytes).expect("a valid trailer round-trips");
    assert!(
        stark_verify_ext_blown_bound(&air, &parsed, 32, 16, 3, context),
        "the parsed money-grade attestation was rejected"
    );
    assert!(
        !stark_verify_ext_blown_bound(&air, &parsed, 32, 16, 3, b"capsule:other:v1"),
        "the parsed attestation passed under the wrong identity"
    );
    assert!(
        deserialize_proof_ext(&bytes[..bytes.len() / 2]).is_none(),
        "a truncated trailer parsed instead of failing"
    );
}

// The full loop: the enrollment tool builds a capsule's trailer, and the kernel
// gate's exact parse-and-verify accepts it under the capsule identity and refuses it
// under any other. This is the prover side meeting the verifier side on the same
// byte layout, which is what makes the gate usable end to end.
#[test]
fn a_built_trailer_is_accepted_by_the_gate_logic() {
    use crate::crypto::stark::air::{
        build_attestation_trailer, deserialize_proof_ext, measure_capsule,
        stark_verify_ext_blown_bound, MerkleMembership,
    };
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let images: [&[u8]; 4] = [b"cap:a bytes", b"cap:b bytes", b"cap:c bytes", b"cap:d bytes"];
    let index = 1usize;
    let context = b"capsule:b:v1";

    // Tool side.
    let trailer =
        build_attestation_trailer(&hasher, log_rounds, &images, index, context, 32, 16, 3);

    // Kernel side: the same parse the spawn gate runs.
    let depth = trailer[8] as usize;
    let mut siblings = Vec::with_capacity(depth);
    for i in 0..depth {
        let mut s = [Fp::ZERO; RATE];
        for (j, lane) in s.iter_mut().enumerate() {
            let mut w = [0u8; 8];
            w.copy_from_slice(&trailer[9 + i * 32 + j * 8..9 + i * 32 + j * 8 + 8]);
            *lane = Fp::from_u64(u64::from_le_bytes(w));
        }
        siblings.push(s);
    }
    let sib_end = 9 + depth * 32;
    let dir_bytes = depth.div_ceil(8);
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let directions: Vec<bool> = (0..depth).map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1).collect();
    let proof = deserialize_proof_ext(&trailer[sib_end + dir_bytes..]).expect("the proof parses");

    // The kernel's own policy root, which enrollment publishes.
    let leaves: Vec<[Fp; RATE]> = images.iter().map(|i| measure_capsule(&hasher, i)).collect();
    let root = PoseidonMerkleTree::commit(&hasher, &leaves).root();
    let air = MerkleMembership::new(hasher.clone(), log_rounds, root, siblings, directions);

    assert!(
        stark_verify_ext_blown_bound(&air, &proof, 32, 16, 3, context),
        "a tool-built trailer was rejected by the gate logic"
    );
    assert!(
        !stark_verify_ext_blown_bound(&air, &proof, 32, 16, 3, b"capsule:evil:v1"),
        "the trailer passed under the wrong capsule identity"
    );
}

// The shared verify core, exercised the way the kernel self-attestation calls it:
// a kernel image is measured into the context, a trailer proves its measurement is
// enrolled under the trust root, and the core accepts it and refuses a foreign boot
// context. Same path the capsule gate uses, one layer up.
#[test]
fn the_shared_verify_core_accepts_a_kernel_self_attestation() {
    use crate::crypto::stark::air::{
        build_attestation_trailer, measure_capsule, verify_membership_trailer, RATE,
    };
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    // The enrolled kernel image plus a few others, its measurement in the root.
    let images: [&[u8]; 4] = [b"nonos-kernel image", b"other:a", b"other:b", b"other:c"];
    let index = 0usize;
    let boot_ctx = b"kernel:boot:epoch:1";

    let trailer =
        build_attestation_trailer(&hasher, log_rounds, &images, index, boot_ctx, 32, 16, 3);

    // The trust root the boot chain carries, as 32 bytes.
    let leaves: Vec<[Fp; RATE]> = images.iter().map(|i| measure_capsule(&hasher, i)).collect();
    let root_rate = PoseidonMerkleTree::commit(&hasher, &leaves).root();
    let mut root = [0u8; 32];
    for (i, lane) in root_rate.iter().enumerate() {
        root[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }

    let depth = trailer[8] as usize;
    assert!(
        verify_membership_trailer(&hasher, log_rounds, root, depth, &trailer, boot_ctx, 32, 16, 3),
        "the kernel self-attestation was rejected by the shared core"
    );
    assert!(
        !verify_membership_trailer(
            &hasher,
            log_rounds,
            root,
            depth,
            &trailer,
            b"kernel:boot:epoch:2",
            32,
            16,
            3
        ),
        "a self-attestation passed under the wrong boot context"
    );
}

#[test]
fn membership_proofs_verify_at_fri_layer_depths() {
    // FRI layers are sized 2^k, so paths are depth k, any value. The AIR pads its
    // slots to a power of two, so an opening from a FRI-sized layer (depth 5, the
    // same Poseidon commitment the recursion-ready FRI uses) proves in a STARK.
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    for &(count, index) in &[(16usize, 9usize), (32, 21), (64, 40)] {
        assert!(
            prove_membership(&hasher, &merkle_leaves(count), index, log_rounds),
            "membership at a {count}-leaf layer rejected"
        );
    }
}

/// Build the batched trace: each opening runs its Merkle path, then the state
/// resets to the next opening's leaf; padding openings run freely.
fn opening_at(tree: &PoseidonMerkleTree, leaves: &[[Fp; RATE]], index: usize) -> Opening {
    let path = tree.open(index);
    let directions = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    Opening { leaf: leaves[index], root: tree.root(), siblings: path, directions }
}

#[test]
fn a_batched_opening_proof_verifies() {
    // Verify two openings of one FRI layer (the a and b a query reads) in a
    // single STARK: the heavy half of a FRI query verifier.
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let openings = alloc::vec![opening_at(&tree, &leaves, 2), opening_at(&tree, &leaves, 6)];
    let air = MultiMembership::new(hasher.clone(), log_rounds, openings);
    let trace = air.trace();
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "a batched opening proof was rejected");
}

#[test]
fn a_batched_opening_with_a_wrong_root_is_rejected() {
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let mut o0 = opening_at(&tree, &leaves, 2);
    let o1 = opening_at(&tree, &leaves, 6);
    o0.root[0] = o0.root[0] + Fp::ONE; // corrupt the first opening's claimed root
    let air = MultiMembership::new(hasher.clone(), log_rounds, alloc::vec![o0, o1]);
    // The trace hashes the true leaves and siblings, so its checkpoint holds the
    // real root while the boundary pins the corrupted one: a mismatch.
    let trace = air.trace();
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a wrong batched root verified");
}

fn value_leaves(values: &[Fp]) -> Vec<[Fp; RATE]> {
    values
        .iter()
        .map(|v| {
            let mut d = [Fp::ZERO; RATE];
            d[0] = *v;
            d
        })
        .collect()
}

#[test]
fn a_full_fri_query_verifies() {
    // A whole FRI query: fold a codeword, and for each layer prove its two
    // openings are committed with the batched-opening STARK, then check the fold
    // is consistent. The expensive Merkle work is proven; the cheap fold is a
    // public field check. This is FRI query verification, composed from step 2.
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let (k, n_folds) = (5u32, 4usize); // domain 32, fold to size 2
    let n = 1usize << k;
    let inv2 = Fp::from_u64(2).inv();
    let base_omega = root_of_unity(k);
    let shift = Fp::from_u64(7);

    // Fold a codeword, keeping every layer and its Poseidon commitment.
    let mut s = 0xf17_u64 | 1;
    let mut layers: Vec<Vec<Fp>> = alloc::vec![(0..n).map(|_| Fp::from_u64(xs(&mut s))).collect()];
    let mut betas: Vec<Fp> = Vec::new();
    let (mut omega, mut coset) = (base_omega, shift);
    for _ in 0..n_folds {
        let beta = Fp::from_u64(xs(&mut s));
        betas.push(beta);
        let cur = layers.last().unwrap().clone();
        let half = cur.len() / 2;
        let mut next = Vec::with_capacity(half);
        let mut x = coset;
        for i in 0..half {
            let (a, b) = (cur[i], cur[i + half]);
            next.push((a + b) * inv2 + beta * ((a - b) * inv2 * x.inv()));
            x = x * omega;
        }
        layers.push(next);
        omega = omega.square();
        coset = coset.square();
    }

    let q = 6usize;
    let (mut om, mut cs) = (base_omega, shift);
    for m in 0..n_folds {
        let size = layers[m].len();
        let half = size / 2;
        let i = q % half;
        let (a, b) = (layers[m][i], layers[m][i + half]);

        // Prove both openings are committed under the layer's root.
        let leaves = value_leaves(&layers[m]);
        let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
        let openings =
            alloc::vec![opening_at(&tree, &leaves, i), opening_at(&tree, &leaves, i + half)];
        let air = MultiMembership::new(hasher.clone(), log_rounds, openings);
        let trace = air.trace();
        let proof = stark_prove(&air, &trace, QUERIES);
        assert!(stark_verify(&air, &proof, QUERIES), "layer {m} openings not proven committed");

        // Check the fold publicly: it must land on the next layer's value.
        let x = cs * om.pow(i as u64);
        let folded = (a + b) * inv2 + betas[m] * ((a - b) * inv2 * x.inv());
        assert_eq!(folded, layers[m + 1][i], "fold at layer {m} inconsistent");
        om = om.square();
        cs = cs.square();
    }
}

/// Build the grand-product column: start at one, multiply by (a+g)/(b+g) per
/// step over the sequence, then carry the final value through the inert tail.
fn permutation_trace(a: &[Fp], b: &[Fp], gamma: Fp) -> Vec<Fp> {
    let n = a.len();
    let total = 2 * n;
    let mut z = alloc::vec![Fp::ZERO; total];
    z[0] = Fp::ONE;
    for i in 0..n {
        z[i + 1] = z[i] * (a[i] + gamma) * (b[i] + gamma).inv();
    }
    for i in n..total - 1 {
        z[i + 1] = z[i];
    }
    z
}

#[test]
fn a_copy_constraint_verifies() {
    // sigma has one non-trivial cycle {0, 3}: the wiring requires the values at
    // positions 0 and 3 to be equal. This is how a beta computed in one region
    // is bound to where a fold consumes it in another.
    let sigma = alloc::vec![3usize, 1, 2, 0, 4, 5, 6, 7];
    let values: Vec<Fp> = [5u64, 1, 2, 5, 8, 9, 10, 11].iter().map(|v| Fp::from_u64(*v)).collect();
    let (beta, gamma) = (Fp::from_u64(0x5171), Fp::from_u64(0x9e37));
    let air = CopyConstraint::new(values, sigma, beta, gamma);
    let trace = air.trace();
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest copy constraint was rejected");
}

#[test]
fn a_violated_copy_constraint_is_rejected() {
    // The wiring says positions 0 and 3 are equal, but they are not.
    let sigma = alloc::vec![3usize, 1, 2, 0, 4, 5, 6, 7];
    let values: Vec<Fp> = [5u64, 1, 2, 9, 8, 9, 10, 11].iter().map(|v| Fp::from_u64(*v)).collect();
    let (beta, gamma) = (Fp::from_u64(0x5171), Fp::from_u64(0x9e37));
    let air = CopyConstraint::new(values, sigma, beta, gamma);
    let trace = air.trace();
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a violated copy constraint verified");
}

#[test]
fn a_permutation_argument_verifies() {
    // Two sequences with the same multiset: the grand product returns to one.
    let a: Vec<Fp> = (1..=8).map(Fp::from_u64).collect();
    let b: Vec<Fp> = [3u64, 1, 4, 8, 2, 7, 5, 6].iter().map(|v| Fp::from_u64(*v)).collect();
    let gamma = Fp::from_u64(0x9e37_79b9);
    let trace = permutation_trace(&a, &b, gamma);
    let air = Permutation::new(a, b, gamma);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest permutation was rejected");
}

#[test]
fn a_non_permutation_is_rejected() {
    // Different multisets: the product does not return to one, so the checkpoint
    // fails and the proof is rejected.
    let a: Vec<Fp> = (1..=8).map(Fp::from_u64).collect();
    let b: Vec<Fp> = (2..=9).map(Fp::from_u64).collect();
    let gamma = Fp::from_u64(0x9e37_79b9);
    let trace = permutation_trace(&a, &b, gamma);
    let air = Permutation::new(a, b, gamma);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a non-permutation verified");
}

#[test]
fn a_membership_proof_for_a_wrong_root_is_rejected() {
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let index = 5usize;
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);

    let mut wrong = tree.root();
    wrong[0] = wrong[0] + Fp::ONE;
    let air = MerkleMembership::new(hasher.clone(), log_rounds, wrong, path, directions);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a wrong-root membership proof verified");
}

/// Run the Poseidon sponge transcript: seed with the first value, then permute
/// and absorb each remaining value, and squeeze the first lane. Returns the
/// trace and the challenge.
fn fiat_shamir_trace(
    hasher: &Poseidon,
    inputs: &[Fp],
    log_rounds: u32,
    log_slots: u32,
) -> (Vec<Fp>, Fp) {
    let l = 1usize << log_rounds;
    let blocks = (1usize << log_slots) - 1;
    let mut rows: Vec<[Fp; WIDTH]> = Vec::with_capacity((blocks + 1) * l);
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = inputs[0];
    for k in 0..blocks {
        for round in 0..l {
            rows.push(state);
            state = hasher.round_with_rc(&state, &hasher.round_constant(round));
        }
        if k + 1 < blocks {
            state[0] = state[0] + inputs[k + 1];
        }
    }
    let challenge = state[0];
    for round in 0..l {
        rows.push(state);
        state = hasher.round_with_rc(&state, &hasher.round_constant(round));
    }
    let mut trace = Vec::with_capacity(rows.len() * WIDTH);
    for row in &rows {
        trace.extend_from_slice(row);
    }
    (trace, challenge)
}

#[test]
fn a_fiat_shamir_transcript_verifies() {
    // Prove a challenge was squeezed from a sequence of absorbed values through a
    // Poseidon transcript: challenge derivation, arithmetized, the last piece a
    // recursive verifier needs to run its own Fiat-Shamir in circuit.
    let (log_rounds, log_slots) = (3u32, 2u32); // 8-round permute, 3 absorbs
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let inputs = alloc::vec![Fp::from_u64(111), Fp::from_u64(222), Fp::from_u64(333)];
    let (trace, challenge) = fiat_shamir_trace(&hasher, &inputs, log_rounds, log_slots);

    let air = FiatShamir::new(
        Poseidon::new(log_rounds, [Fp::ZERO; RATE]),
        log_rounds,
        log_slots,
        inputs.clone(),
        challenge,
    );
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest transcript was rejected");

    let bad = FiatShamir::new(
        Poseidon::new(log_rounds, [Fp::ZERO; RATE]),
        log_rounds,
        log_slots,
        inputs,
        challenge + Fp::ONE,
    );
    let bad_proof = stark_prove(&bad, &trace, QUERIES);
    assert!(!stark_verify(&bad, &bad_proof, QUERIES), "a wrong challenge verified");
}

#[test]
fn a_fri_fold_chain_verifies() {
    // Fold a real codeword four times, extract one query's path down the layers,
    // and prove inside a STARK that the folds are consistent and reach the
    // committed final value. This is the FRI verifier's fold check, arithmetized.
    let (k, n_folds, log_layers) = (5u32, 4usize, 3u32); // domain 32, fold to size 2
    let n = 1usize << k;
    let inv2 = Fp::from_u64(2).inv();
    let base_omega = root_of_unity(k);
    let shift = Fp::from_u64(7);

    let mut s = 0xf01d_1234u64 | 1;
    let mut layers: Vec<Vec<Fp>> = Vec::new();
    let mut betas: Vec<Fp> = Vec::new();
    let mut cur: Vec<Fp> = (0..n).map(|_| Fp::from_u64(xs(&mut s))).collect();
    layers.push(cur.clone());
    let mut omega = base_omega;
    let mut coset = shift;
    for _ in 0..n_folds {
        let beta = Fp::from_u64(xs(&mut s));
        betas.push(beta);
        let half = cur.len() / 2;
        let mut next = Vec::with_capacity(half);
        let mut x = coset;
        for i in 0..half {
            let (a, b) = (cur[i], cur[i + half]);
            next.push((a + b) * inv2 + beta * ((a - b) * inv2 * x.inv()));
            x = x * omega;
        }
        cur = next;
        layers.push(cur.clone());
        omega = omega.square();
        coset = coset.square();
    }

    // Extract query q's path (q even so the last fold lands in the first slot).
    let q = 6usize;
    let rows = 1usize << log_layers;
    let mut trace = alloc::vec![Fp::ZERO; rows * 2];
    let (mut x_inv, mut beta_col, mut dir) = (Vec::new(), Vec::new(), Vec::new());
    let mut om = base_omega;
    let mut cs = shift;
    for m in 0..n_folds {
        let half = layers[m].len() / 2;
        let i = q % half;
        trace[m * 2] = layers[m][i];
        trace[m * 2 + 1] = layers[m][i + half];
        x_inv.push((cs * om.pow(i as u64)).inv());
        beta_col.push(betas[m]);
        dir.push(i >= half / 2);
        om = om.square();
        cs = cs.square();
    }
    // Final layer row: its pair, first slot is the committed value.
    trace[n_folds * 2] = layers[n_folds][0];
    trace[n_folds * 2 + 1] = layers[n_folds][1];
    let final_value = layers[n_folds][0];

    let air = FriFold::new(
        log_layers,
        n_folds,
        x_inv.clone(),
        beta_col.clone(),
        dir.clone(),
        final_value,
    );
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest fri fold chain was rejected");

    // Tamper one opened value: the fold no longer lands on the next layer.
    let mut bad = trace.clone();
    bad[2] = bad[2] + Fp::ONE;
    let bad_air = FriFold::new(log_layers, n_folds, x_inv, beta_col, dir, final_value);
    let bad_proof = stark_prove(&bad_air, &bad, QUERIES);
    assert!(!stark_verify(&bad_air, &bad_proof, QUERIES), "a broken fold chain verified");
}

/// One FRI query's fold path: fold a real codeword four times, extract the query
/// column down the layers, and return the fold trace with its AIR. The shape the
/// fold half of a query verifier proves.
fn fri_fold_region(query: usize) -> (Vec<Fp>, FriFold) {
    let (k, n_folds, log_layers) = (5u32, 4usize, 3u32);
    let n = 1usize << k;
    let inv2 = Fp::from_u64(2).inv();
    let base_omega = root_of_unity(k);
    let shift = Fp::from_u64(7);

    let mut s = 0xf01d_1234u64 | 1;
    let mut layers: Vec<Vec<Fp>> = Vec::new();
    let mut betas: Vec<Fp> = Vec::new();
    let mut cur: Vec<Fp> = (0..n).map(|_| Fp::from_u64(xs(&mut s))).collect();
    layers.push(cur.clone());
    let mut omega = base_omega;
    let mut coset = shift;
    for _ in 0..n_folds {
        let beta = Fp::from_u64(xs(&mut s));
        betas.push(beta);
        let half = cur.len() / 2;
        let mut next = Vec::with_capacity(half);
        let mut x = coset;
        for i in 0..half {
            let (a, b) = (cur[i], cur[i + half]);
            next.push((a + b) * inv2 + beta * ((a - b) * inv2 * x.inv()));
            x = x * omega;
        }
        cur = next;
        layers.push(cur.clone());
        omega = omega.square();
        coset = coset.square();
    }

    let rows = 1usize << log_layers;
    let mut trace = alloc::vec![Fp::ZERO; rows * 2];
    let (mut x_inv, mut beta_col, mut dir) = (Vec::new(), Vec::new(), Vec::new());
    let mut om = base_omega;
    let mut cs = shift;
    for m in 0..n_folds {
        let half = layers[m].len() / 2;
        let i = query % half;
        trace[m * 2] = layers[m][i];
        trace[m * 2 + 1] = layers[m][i + half];
        x_inv.push((cs * om.pow(i as u64)).inv());
        beta_col.push(betas[m]);
        dir.push(i >= half / 2);
        om = om.square();
        cs = cs.square();
    }
    trace[n_folds * 2] = layers[n_folds][0];
    trace[n_folds * 2 + 1] = layers[n_folds][1];
    let final_value = layers[n_folds][0];

    (trace, FriFold::new(log_layers, n_folds, x_inv, beta_col, dir, final_value))
}

/// Build the Merkle-opening region for `index` in an eight-leaf tree.
fn merkle_region(index: usize, log_rounds: u32) -> (Vec<Fp>, MerkleMembership) {
    // The hasher runs 2^log_rounds rounds, so its compression matches the AIR's
    // per-slot round count and the committed root equals the trace's final digest.
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let root = tree.root();
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher, log_rounds, root, path, directions);
    (trace, air)
}

#[test]
fn a_fri_query_verifier_is_fused_into_one_proof() {
    // The two halves of a FRI query check, a Merkle opening under the committed
    // root and the fold consistency down the layers, are different-width AIRs.
    // Fused, they are proven and verified as a single STARK: the verification
    // cost of the whole query verifier stays that of one proof.
    let (mem_trace, mem) = merkle_region(3, 3);
    let (fold_trace, fold) = fri_fold_region(6);

    let regions: Vec<Box<dyn Air>> = alloc::vec![Box::new(mem), Box::new(fold)];
    let fused = Fused::new(regions);
    let witness = fused.trace(&[mem_trace, fold_trace]);
    let proof = stark_prove(&fused, &witness, QUERIES);
    assert!(stark_verify(&fused, &proof, QUERIES), "the fused query verifier was rejected");
}

#[test]
fn a_tampered_region_breaks_the_fused_proof() {
    // Corrupt one opened value in the fold region of the fused trace. The single
    // proof must fail: a fault in any region breaks the whole verification.
    let (mem_trace, mem) = merkle_region(3, 3);
    let (fold_trace, fold) = fri_fold_region(6);

    let mem_rows = 1usize << mem.log_trace_len();
    let regions: Vec<Box<dyn Air>> = alloc::vec![Box::new(mem), Box::new(fold)];
    let fused = Fused::new(regions);
    let mut witness = fused.trace(&[mem_trace, fold_trace]);
    // The fold region starts after the membership region; corrupt its first cell.
    let width = 8usize;
    witness[mem_rows * width] = witness[mem_rows * width] + Fp::ONE;
    let proof = stark_prove(&fused, &witness, QUERIES);
    assert!(!stark_verify(&fused, &proof, QUERIES), "a tampered fused region verified");
}

#[test]
fn a_tampered_ood_frame_is_rejected() {
    // The out-of-domain frame is the point where the constraints are actually
    // checked. A frame that lies about the trace breaks the DEEP quotients, and
    // the low-degree test rejects.
    let seed = Fp::from_u64(3);
    let air = Squaring { log_t: 3, seed };
    let mut proof = stark_prove(&air, &squaring_trace(3, seed), QUERIES);
    proof.ood_frame[0] = proof.ood_frame[0] + Fp::ONE;
    assert!(!stark_verify(&air, &proof, QUERIES), "a tampered ood frame verified");
}

#[test]
fn a_value_is_bound_across_two_fused_regions() {
    // Region A computes a value; region B starts from it. A copy constraint over
    // column zero forces A's last cell to equal B's first, so the two regions,
    // each internally valid, must agree on the shared value. This is how a
    // transcript's squeezed challenge binds to where a fold consumes it.
    let a_trace = squaring_trace(3, Fp::from_u64(3));
    let handoff = a_trace[7];
    let b_trace = squaring_trace(3, handoff);

    let mut sigma: Vec<usize> = (0..16).collect();
    sigma.swap(7, 8); // A's last cell (row 7) wired to B's first (row 8)

    let regions: Vec<Box<dyn Air>> = alloc::vec![
        Box::new(Squaring { log_t: 3, seed: Fp::from_u64(3) }),
        Box::new(Squaring { log_t: 3, seed: handoff }),
    ];
    let wired = Wired::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[a_trace, b_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "an honest cross-region binding was rejected");
}

#[test]
fn a_broken_cross_region_binding_is_rejected() {
    // Region B starts from a different value than A produced. Each region is
    // internally valid, but the wiring forces the shared cell equal, so the
    // single proof must fail.
    let a_trace = squaring_trace(3, Fp::from_u64(3));
    let handoff = a_trace[7];
    let wrong = handoff + Fp::ONE;
    let b_trace = squaring_trace(3, wrong);

    let mut sigma: Vec<usize> = (0..16).collect();
    sigma.swap(7, 8);

    let regions: Vec<Box<dyn Air>> = alloc::vec![
        Box::new(Squaring { log_t: 3, seed: Fp::from_u64(3) }),
        Box::new(Squaring { log_t: 3, seed: wrong }),
    ];
    let wired = Wired::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[a_trace, b_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "a broken cross-region binding verified");
}

/// One FRI query's fold path, split into the pieces an in-circuit fold witnesses:
/// the per-layer challenge, the opened pairs, the public inverse points and
/// position bits, and the committed final value.
#[allow(clippy::type_complexity)]
fn trace_fold_data(
    query: usize,
) -> (Vec<Fp>, Vec<Fp>, Vec<Fp>, Vec<Fp>, Vec<bool>, Fp, u32, usize) {
    trace_fold_data_seeded(query, 0xf01d_1234u64 | 1)
}

/// The same fold path over a codeword seeded by `seed`; a different seed folds a
/// different codeword with a different challenge set.
#[allow(clippy::type_complexity)]
fn trace_fold_data_seeded(
    query: usize,
    seed: u64,
) -> (Vec<Fp>, Vec<Fp>, Vec<Fp>, Vec<Fp>, Vec<bool>, Fp, u32, usize) {
    trace_fold_data_seeded_first(query, seed, None)
}

// The same seeded fold, but with an optional first-layer challenge. A recursive
// verifier's FRI fold must run on the challenge its transcript squeezed, so the
// monolith overrides `beta[0]` with that challenge and wires the two together.
#[allow(clippy::type_complexity)]
fn trace_fold_data_seeded_first(
    query: usize,
    seed: u64,
    first_beta: Option<Fp>,
) -> (Vec<Fp>, Vec<Fp>, Vec<Fp>, Vec<Fp>, Vec<bool>, Fp, u32, usize) {
    let (k, n_folds, log_layers) = (5u32, 4usize, 3u32);
    let n = 1usize << k;
    let inv2 = Fp::from_u64(2).inv();
    let base_omega = root_of_unity(k);
    let shift = Fp::from_u64(7);

    let mut s = seed | 1;
    let mut layers: Vec<Vec<Fp>> = Vec::new();
    let mut betas: Vec<Fp> = Vec::new();
    let mut cur: Vec<Fp> = (0..n).map(|_| Fp::from_u64(xs(&mut s))).collect();
    layers.push(cur.clone());
    let mut omega = base_omega;
    let mut coset = shift;
    for layer_i in 0..n_folds {
        let beta = match first_beta {
            Some(b0) if layer_i == 0 => b0,
            _ => Fp::from_u64(xs(&mut s)),
        };
        betas.push(beta);
        let half = cur.len() / 2;
        let mut next = Vec::with_capacity(half);
        let mut x = coset;
        for i in 0..half {
            let (a, b) = (cur[i], cur[i + half]);
            next.push((a + b) * inv2 + beta * ((a - b) * inv2 * x.inv()));
            x = x * omega;
        }
        cur = next;
        layers.push(cur.clone());
        omega = omega.square();
        coset = coset.square();
    }

    let (mut a, mut b, mut x_inv, mut dir) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut om = base_omega;
    let mut cs = shift;
    for layer in layers.iter().take(n_folds) {
        let half = layer.len() / 2;
        let i = query % half;
        a.push(layer[i]);
        b.push(layer[i + half]);
        x_inv.push((cs * om.pow(i as u64)).inv());
        dir.push(i >= half / 2);
        om = om.square();
        cs = cs.square();
    }
    a.push(layers[n_folds][0]);
    b.push(layers[n_folds][1]);
    let final_value = layers[n_folds][0];
    (betas, a, b, x_inv, dir, final_value, log_layers, n_folds)
}

#[test]
fn an_in_circuit_fold_verifies() {
    // The fold with its folding challenge witnessed in column zero, proven the
    // same as the public-challenge fold. This is the shape the monolith wires.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let air = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let trace = air.trace(&beta, &a, &b);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest in-circuit fold was rejected");
}

#[test]
fn a_corrupted_in_circuit_fold_is_rejected() {
    let (beta, mut a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    a[0] = a[0] + Fp::ONE; // an opened value that no longer folds
    let air = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let trace = air.trace(&beta, &a, &b);
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a broken in-circuit fold verified");
}

#[test]
fn a_fold_bound_to_its_challenge_source_verifies() {
    // A supplier region produces the first folding challenge; the in-circuit fold
    // consumes it. The wiring forces the fold to run on exactly the supplied
    // challenge: the transcript-to-fold binding on a real fold.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let source = squaring_trace(3, beta[0]); // column zero holds beta[0] at row 0
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let mut sigma: Vec<usize> = (0..16).collect();
    sigma.swap(0, 8); // source row 0 wired to fold row 0 (fused row 8)

    let regions: Vec<Box<dyn Air>> =
        alloc::vec![Box::new(Squaring { log_t: 3, seed: beta[0] }), Box::new(fold),];
    let wired = Wired::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "a fold bound to its challenge was rejected");
}

#[test]
fn a_fold_using_the_wrong_challenge_is_rejected() {
    // The fold is internally valid and the supplier is internally valid, but the
    // fold's challenge is not the one the supplier produced. The wiring rejects.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let source = squaring_trace(3, beta[0] + Fp::ONE); // supplies a different value
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let mut sigma: Vec<usize> = (0..16).collect();
    sigma.swap(0, 8);

    let regions: Vec<Box<dyn Air>> =
        alloc::vec![Box::new(Squaring { log_t: 3, seed: beta[0] + Fp::ONE }), Box::new(fold),];
    let wired = Wired::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "a fold on the wrong challenge verified");
}

#[test]
fn a_fold_bound_to_both_its_challenge_and_opening_verifies() {
    // The monolith's per-query binding: the fold must run on the transcript's
    // challenge AND the opening's revealed value. A width-two source supplies
    // both in one row; the wiring binds column zero (the challenge) and column
    // one (the opened value) at once.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (rc0, rc1) = (Fp::from_u64(13), Fp::from_u64(17));
    let (source, out) = permutation2_trace(3, beta[0], a[0], rc0, rc1);
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(0, 16); // source row 0 col 0 (challenge) <-> fold row 0 col 0
    sigma.swap(1, 17); // source row 0 col 1 (opening)   <-> fold row 0 col 1

    let regions: Vec<Box<dyn Air>> =
        alloc::vec![Box::new(Permutation2 { log_t: 3, rc0, rc1, out }), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(
        stark_verify(&wired, &proof, QUERIES),
        "a fold bound to challenge and opening was rejected"
    );
}

#[test]
fn a_fold_bound_to_a_wrong_opening_is_rejected() {
    // The source supplies the right challenge but a different opened value; the
    // multi-column wiring catches the opening even though the challenge matches.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (rc0, rc1) = (Fp::from_u64(13), Fp::from_u64(17));
    let (source, out) = permutation2_trace(3, beta[0], a[0] + Fp::ONE, rc0, rc1);
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(0, 16);
    sigma.swap(1, 17);

    let regions: Vec<Box<dyn Air>> =
        alloc::vec![Box::new(Permutation2 { log_t: 3, rc0, rc1, out }), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "a fold on a wrong opening verified");
}

/// A single Merkle opening whose committed leaf is the scalar `v` (a FRI leaf is
/// `[v, 0, 0, 0]`), at an even index so the scalar lands in column zero. Returns
/// the opening trace, the AIR, and its `opened_cells()` map.
fn opening_of_scalar(v: Fp, log_rounds: u32) -> (Vec<Fp>, MultiMembership, Vec<(usize, usize)>) {
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let mut leaves = merkle_leaves(4);
    leaves[2] = [v, Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let opening = opening_at(&tree, &leaves, 2);
    let mem = MultiMembership::new(hasher, log_rounds, alloc::vec![opening]);
    let trace = mem.trace();
    let cells = mem.opened_cells();
    (trace, mem, cells)
}

#[test]
fn a_fold_bound_to_its_committed_opening_verifies() {
    // The other half of the monolith's per-query binding: the fold must fold the
    // value the Merkle opening actually committed, not an arbitrary one. The
    // opening commits `a[0]` as its leaf; the wiring binds that leaf cell to the
    // fold's opened value.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (mem_trace, mem, cells) = opening_of_scalar(a[0], 2);
    assert_eq!(cells[0].1, 0, "the committed scalar should sit in column zero");
    let mem_h = mem.rows();
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let fold_h = 1usize << log_layers;
    let (k, span) = (2usize, (mem_h + fold_h).next_power_of_two());
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // leaf scalar at (cells[0].0, col 0) <-> fold's opened value at (mem_h, col 1)
    sigma.swap(cells[0].0 * k, mem_h * k + 1);

    let regions: Vec<Box<dyn Air>> = alloc::vec![Box::new(mem), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[mem_trace, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "a fold bound to its opening was rejected");
}

#[test]
fn a_fold_folding_an_uncommitted_value_is_rejected() {
    // The opening commits a different value than the fold folds. Each is
    // internally valid, but the wiring forces the fold to fold exactly what was
    // committed, so the single proof fails.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (mem_trace, mem, cells) = opening_of_scalar(a[0] + Fp::ONE, 2);
    let mem_h = mem.rows();
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let fold_h = 1usize << log_layers;
    let (k, span) = (2usize, (mem_h + fold_h).next_power_of_two());
    let mut sigma: Vec<usize> = (0..span * k).collect();
    sigma.swap(cells[0].0 * k, mem_h * k + 1);

    let regions: Vec<Box<dyn Air>> = alloc::vec![Box::new(mem), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[mem_trace, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "a fold on an uncommitted value verified");
}

#[test]
fn a_full_per_query_verifier_is_one_stark() {
    // The monolith, per query: a challenge source, the Merkle opening of the
    // codeword value, and the in-circuit fold, fused into one trace and verified
    // as a single STARK. The wiring forces the fold to run on exactly the
    // challenge the source produced AND exactly the value the opening committed.
    // One constant-size proof stands for the whole per-query FRI check.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (mem_trace, mem, cells) = opening_of_scalar(a[0], 2);
    let source = squaring_trace(3, beta[0]); // column zero row 0 holds beta[0]
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    // Region offsets: source [0,8), opening [8,24), fold [24,32).
    let src_h = 1usize << 3;
    let mem_h = mem.rows();
    let fold_off = src_h + mem_h;
    let (k, span) = (2usize, (src_h + mem_h + (1usize << log_layers)).next_power_of_two());
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // source.beta (row 0, col 0) <-> fold.beta (fold_off, col 0)
    sigma.swap(0, fold_off * k);
    // opening leaf (src_h + cells[0].0, col 0) <-> fold.a (fold_off, col 1)
    sigma.swap((src_h + cells[0].0) * k, fold_off * k + 1);

    let regions: Vec<Box<dyn Air>> =
        alloc::vec![Box::new(Squaring { log_t: 3, seed: beta[0] }), Box::new(mem), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, mem_trace, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "the fused per-query verifier was rejected");
}

#[test]
fn a_per_query_verifier_rejects_a_wrong_challenge() {
    // Same fused per-query verifier, but the source produces a challenge the fold
    // did not use. One region disagrees on a wired cell, so the whole proof fails.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (mem_trace, mem, cells) = opening_of_scalar(a[0], 2);
    let source = squaring_trace(3, beta[0] + Fp::ONE); // wrong challenge
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let src_h = 1usize << 3;
    let mem_h = mem.rows();
    let fold_off = src_h + mem_h;
    let (k, span) = (2usize, (src_h + mem_h + (1usize << log_layers)).next_power_of_two());
    let mut sigma: Vec<usize> = (0..span * k).collect();
    sigma.swap(0, fold_off * k);
    sigma.swap((src_h + cells[0].0) * k, fold_off * k + 1);

    let regions: Vec<Box<dyn Air>> = alloc::vec![
        Box::new(Squaring { log_t: 3, seed: beta[0] + Fp::ONE }),
        Box::new(mem),
        Box::new(fold)
    ];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, mem_trace, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(
        !stark_verify(&wired, &proof, QUERIES),
        "a per-query verifier accepted a wrong challenge"
    );
}

#[test]
fn a_per_query_verifier_rejects_a_wrong_opening() {
    // The opening commits a value the fold did not fold. The proof fails.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (mem_trace, mem, cells) = opening_of_scalar(a[0] + Fp::ONE, 2); // commits a wrong value
    let source = squaring_trace(3, beta[0]);
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let src_h = 1usize << 3;
    let mem_h = mem.rows();
    let fold_off = src_h + mem_h;
    let (k, span) = (2usize, (src_h + mem_h + (1usize << log_layers)).next_power_of_two());
    let mut sigma: Vec<usize> = (0..span * k).collect();
    sigma.swap(0, fold_off * k);
    sigma.swap((src_h + cells[0].0) * k, fold_off * k + 1);

    let regions: Vec<Box<dyn Air>> =
        alloc::vec![Box::new(Squaring { log_t: 3, seed: beta[0] }), Box::new(mem), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[source, mem_trace, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(
        !stark_verify(&wired, &proof, QUERIES),
        "a per-query verifier accepted a wrong opening"
    );
}

/// Fuse the in-circuit folds of several FRI queries into one trace and wire, per
/// layer, every query's folding challenge into a single cycle: the copy
/// constraint forces all queries to fold on the same challenge set. Returns the
/// wired AIR and its witness. `seeds[q]` seeds query q's codeword, so an honest
/// fan-out uses one seed for all and a dishonest one gives a query a different
/// challenge set.
fn multi_query_fanout(queries: &[usize], seeds: &[u64]) -> (Wired, Vec<Fp>) {
    let mut regions: Vec<Box<dyn Air>> = Vec::new();
    let mut traces: Vec<Vec<Fp>> = Vec::new();
    let mut n_folds = 0usize;
    let mut height = 0usize;
    for (&query, &seed) in queries.iter().zip(seeds) {
        let (beta, a, b, x_inv, dir, fv, ll, nf) = trace_fold_data_seeded(query, seed);
        n_folds = nf;
        height = 1usize << ll;
        let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
        traces.push(fold.trace(&beta, &a, &b));
        regions.push(Box::new(fold));
    }

    let q_count = queries.len();
    let span = (q_count * height).next_power_of_two();
    let mut sigma: Vec<usize> = (0..span).collect(); // wired_cols = [0], so cell id == row
                                                     // Per layer, cycle the challenge cell across all queries: q -> q+1 -> ... -> 0.
    for m in 0..n_folds {
        for q in 0..q_count {
            let here = q * height + m;
            let next = ((q + 1) % q_count) * height + m;
            sigma[here] = next;
        }
    }

    let wired = Wired::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&traces);
    (wired, witness)
}

#[test]
fn every_query_folds_on_the_same_challenge_set() {
    // Three FRI queries, folded in one STARK, all wired to a single challenge
    // set. The honest fan-out, where every query used the same transcript
    // challenges, verifies.
    let seed = 0xf01d_1234u64 | 1;
    let (wired, witness) = multi_query_fanout(&[6, 10, 2], &[seed, seed, seed]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "an honest multi-query fan-out was rejected");
}

#[test]
fn a_query_folding_on_a_different_challenge_set_is_rejected() {
    // One query folded on a different challenge set than the others. Each fold is
    // internally valid, but the wiring forces one shared set, so the single proof
    // fails.
    let seed = 0xf01d_1234u64 | 1;
    let (wired, witness) = multi_query_fanout(&[6, 10, 2], &[seed, 0xdead_beef, seed]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(
        !stark_verify(&wired, &proof, QUERIES),
        "a query on a different challenge set verified"
    );
}

/// The whole-proof monolith: for each query, a Merkle opening of its codeword
/// value and its in-circuit fold, all fused into one trace. Per query the
/// opening is wired to the fold's opened value; across queries every fold's
/// challenge is wired into one cycle. So one STARK attests, for every query at
/// once, that the fold folded the committed value and that all queries used one
/// challenge set. `seeds[q]` seeds query q; `wrong_opening` makes query 0 commit
/// a value it did not fold.
fn whole_proof_monolith(queries: &[usize], seeds: &[u64], wrong_opening: bool) -> (Wired, Vec<Fp>) {
    let mut regions: Vec<Box<dyn Air>> = Vec::new();
    let mut traces: Vec<Vec<Fp>> = Vec::new();
    let mut heights: Vec<usize> = Vec::new();
    let mut open_idx: Vec<usize> = Vec::new();
    let mut fold_idx: Vec<usize> = Vec::new();
    let mut n_folds = 0usize;

    for (qi, (&query, &seed)) in queries.iter().zip(seeds).enumerate() {
        let (beta, a, b, x_inv, dir, fv, ll, nf) = trace_fold_data_seeded(query, seed);
        n_folds = nf;
        let scalar = if wrong_opening && qi == 0 { a[0] + Fp::ONE } else { a[0] };
        let (mtr, mem, _) = opening_of_scalar(scalar, 2);
        open_idx.push(regions.len());
        heights.push(mem.rows());
        traces.push(mtr);
        regions.push(Box::new(mem));

        let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
        fold_idx.push(regions.len());
        heights.push(1usize << ll);
        traces.push(fold.trace(&beta, &a, &b));
        regions.push(Box::new(fold));
    }

    let mut offsets: Vec<usize> = Vec::new();
    let mut acc = 0usize;
    for &h in &heights {
        offsets.push(acc);
        acc += h;
    }
    let span = acc.next_power_of_two();
    let k = 2usize;
    let mut sigma: Vec<usize> = (0..span * k).collect();

    // Per query: the opening's leaf (column zero) <-> the fold's opened value
    // (column one).
    for qi in 0..queries.len() {
        let leaf = offsets[open_idx[qi]] * k; // (row, col 0)
        let opened = offsets[fold_idx[qi]] * k + 1; // (row, col 1)
        sigma.swap(leaf, opened);
    }
    // Across queries: cycle each layer's folding challenge (column zero).
    let qn = queries.len();
    for m in 0..n_folds {
        for qi in 0..qn {
            let here = (offsets[fold_idx[qi]] + m) * k;
            let next = (offsets[fold_idx[(qi + 1) % qn]] + m) * k;
            sigma[here] = next;
        }
    }

    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&traces);
    (wired, witness)
}

#[test]
fn the_whole_fri_verification_is_one_stark() {
    // Two queries, each with its opening and its fold, all in one proof: every
    // fold folded the committed value and both queries used one challenge set.
    let seed = 0xf01d_1234u64 | 1;
    let (wired, witness) = whole_proof_monolith(&[6, 10], &[seed, seed], false);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "the whole-proof monolith was rejected");
}

#[test]
fn the_monolith_rejects_an_uncommitted_fold() {
    // One query folds a value its opening did not commit.
    let seed = 0xf01d_1234u64 | 1;
    let (wired, witness) = whole_proof_monolith(&[6, 10], &[seed, seed], true);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "the monolith accepted an uncommitted fold");
}

#[test]
fn the_monolith_rejects_a_split_challenge_set() {
    // The two queries fold on different challenge sets.
    let seed = 0xf01d_1234u64 | 1;
    let (wired, witness) = whole_proof_monolith(&[6, 10], &[seed, 0xdead_beef], false);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "the monolith accepted a split challenge set");
}

#[test]
fn the_fan_out_wiring_is_robust_under_fuzzing() {
    // The wiring invariant across the input space, not just hand-picked cases:
    // over many random query sets, an honest fan-out (all queries on one
    // challenge set) always verifies, and giving one query a different set is
    // always rejected.
    let mut s = 0x9e37_79b9u64 | 1;
    for _ in 0..24 {
        let q_count = 2 + (xs(&mut s) % 3) as usize; // 2..=4 queries
        let queries: Vec<usize> = (0..q_count).map(|_| (xs(&mut s) % 32) as usize).collect();
        let base = xs(&mut s);

        // Honest: every query shares one challenge set.
        let honest = alloc::vec![base; q_count];
        let (w, wit) = multi_query_fanout(&queries, &honest);
        let p = stark_prove(&w, &wit, QUERIES);
        assert!(stark_verify(&w, &p, QUERIES), "honest fan-out rejected: {queries:?}");

        // Dishonest: one query folds on a different set.
        let victim = (xs(&mut s) % q_count as u64) as usize;
        let mut seeds = honest.clone();
        seeds[victim] = base ^ 0xffff_ffff;
        let (w2, wit2) = multi_query_fanout(&queries, &seeds);
        let p2 = stark_prove(&w2, &wit2, QUERIES);
        assert!(!stark_verify(&w2, &p2, QUERIES), "split set accepted: {queries:?} v{victim}");
    }
}

#[test]
fn both_fold_inputs_are_bound_to_committed_openings() {
    // A FRI query opens both f(x) and f(-x). This binds both of the fold's
    // layer-zero inputs to committed Merkle openings, over three wired columns:
    // the fold folds two values, and both are proven committed, not just the
    // first.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (open_a, mem_a, _) = opening_of_scalar(a[0], 2);
    let (open_b, mem_b, _) = opening_of_scalar(b[0], 2);
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let mem_h = mem_a.rows();
    let fold_off = 2 * mem_h;
    let k = 3usize;
    let span = (2 * mem_h + (1usize << log_layers)).next_power_of_two();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // open_a.leaf (row 0, col 0) <-> fold.a (fold_off, col 1)
    sigma.swap(0, fold_off * k + 1);
    // open_b.leaf (row mem_h, col 0) <-> fold.b (fold_off, col 2)
    sigma.swap(mem_h * k, fold_off * k + 2);

    let regions: Vec<Box<dyn Air>> = alloc::vec![Box::new(mem_a), Box::new(mem_b), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1, 2], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[open_a, open_b, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "a fold on two committed openings was rejected");
}

#[test]
fn a_fold_with_an_uncommitted_second_input_is_rejected() {
    // The first input is committed but the second is a value no opening committed.
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(6);
    let (open_a, mem_a, _) = opening_of_scalar(a[0], 2);
    let (open_b, mem_b, _) = opening_of_scalar(b[0] + Fp::ONE, 2); // commits a wrong b
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    let fold_trace = fold.trace(&beta, &a, &b);

    let mem_h = mem_a.rows();
    let fold_off = 2 * mem_h;
    let k = 3usize;
    let span = (2 * mem_h + (1usize << log_layers)).next_power_of_two();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    sigma.swap(0, fold_off * k + 1);
    sigma.swap(mem_h * k, fold_off * k + 2);

    let regions: Vec<Box<dyn Air>> = alloc::vec![Box::new(mem_a), Box::new(mem_b), Box::new(fold)];
    let wired = Wired::new(regions, alloc::vec![0, 1, 2], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[open_a, open_b, fold_trace]);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(
        !stark_verify(&wired, &proof, QUERIES),
        "a fold on an uncommitted second input verified"
    );
}

/// A minimal single Merkle opening committing the scalar `v` (two leaves, two
/// rounds), so per-layer openings stay cheap to fuse.
fn small_opening_of_scalar(v: Fp) -> (Vec<Fp>, MultiMembership) {
    let log_rounds = 1u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let mut leaves = merkle_leaves(2);
    leaves[0] = [v, Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let opening = opening_at(&tree, &leaves, 0);
    let mem = MultiMembership::new(hasher, log_rounds, alloc::vec![opening]);
    let trace = mem.trace();
    (trace, mem)
}

/// Fuse one opening per fold layer, each committing that layer's opened value,
/// and wire each to the fold's input at that layer. `wrong_layer`, if set, makes
/// that layer's opening commit a value the fold did not fold.
fn per_layer_monolith(query: usize, wrong_layer: Option<usize>) -> (Wired, Vec<Fp>) {
    let (beta, a, b, x_inv, dir, final_value, log_layers, n_folds) = trace_fold_data(query);
    let mut regions: Vec<Box<dyn Air>> = Vec::new();
    let mut traces: Vec<Vec<Fp>> = Vec::new();
    let mut open_rows: Vec<usize> = Vec::new();
    let mut acc = 0usize;
    for (m, &am) in a.iter().enumerate().take(n_folds) {
        let scalar = if wrong_layer == Some(m) { am + Fp::ONE } else { am };
        let (tr, mem) = small_opening_of_scalar(scalar);
        open_rows.push(acc);
        acc += tr.len() / WIDTH;
        traces.push(tr);
        regions.push(Box::new(mem));
    }
    let fold_off = acc;
    let fold = TraceFold::new(log_layers, n_folds, x_inv, dir, final_value);
    traces.push(fold.trace(&beta, &a, &b));
    regions.push(Box::new(fold));
    acc += 1usize << log_layers;

    let span = acc.next_power_of_two();
    let k = 2usize;
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for (m, &orow) in open_rows.iter().enumerate() {
        // opening m's leaf (col 0) <-> fold input at layer m (col 1)
        sigma.swap(orow * k, (fold_off + m) * k + 1);
    }
    let wired = Wired::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&traces);
    (wired, witness)
}

#[test]
fn every_layer_input_is_a_committed_opening() {
    // The full per-query opening structure: every layer's fold input is bound to
    // a committed Merkle opening at that layer, not just the first.
    let (wired, witness) = per_layer_monolith(6, None);
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(stark_verify(&wired, &proof, QUERIES), "per-layer committed openings rejected");
}

#[test]
fn an_uncommitted_layer_input_is_rejected() {
    // One layer folds a value its opening did not commit; the proof fails.
    let (wired, witness) = per_layer_monolith(6, Some(2));
    let proof = stark_prove(&wired, &witness, QUERIES);
    assert!(!stark_verify(&wired, &proof, QUERIES), "an uncommitted layer input verified");
}

// The money-grade composition, compose_ext, evaluates the constraints at the
// out-of-domain point z in Fp2. It must faithfully extend the base compose: on a
// base-embedded point with base-embedded inputs, it returns the embedded base
// result. This ties the Fp2 composition algebra to the already-tested one.
#[test]
fn compose_ext_faithfully_extends_compose() {
    use crate::crypto::stark::air::{compose, compose_ext, Air, Fibonacci};
    use crate::crypto::stark::field::Fp2;
    use crate::crypto::stark::fri::root_of_unity;

    let air = Fibonacci { log_t: 4 };
    let g = root_of_unity(air.log_trace_len());
    let mut s = 0xF1B0u64 | 1;
    let rnd = |s: &mut u64| {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        Fp::from_u64(*s)
    };

    for _ in 0..200 {
        let x = rnd(&mut s);
        let window: alloc::vec::Vec<Fp> = (0..air.window_size()).map(|_| rnd(&mut s)).collect();
        let periodic: alloc::vec::Vec<Fp> = alloc::vec::Vec::new();
        let ncoeff = air.num_transition() + air.boundary().len();
        let coeffs: alloc::vec::Vec<Fp> = (0..ncoeff).map(|_| rnd(&mut s)).collect();

        let base = compose(&air, g, x, &window, &periodic, &coeffs);

        let window_e: alloc::vec::Vec<Fp2> = window.iter().map(|v| Fp2::from_base(*v)).collect();
        let periodic_e: alloc::vec::Vec<Fp2> =
            periodic.iter().map(|v| Fp2::from_base(*v)).collect();
        let coeffs_e: alloc::vec::Vec<Fp2> = coeffs.iter().map(|v| Fp2::from_base(*v)).collect();
        let ext = compose_ext(&air, g, Fp2::from_base(x), &window_e, &periodic_e, &coeffs_e);

        assert_eq!(Fp2::from_base(base), ext, "compose_ext diverged from compose");
    }
}

// The full money-grade DEEP STARK, end to end on a real computation: the OOD point
// is drawn from Fp2, the composition and DEEP polynomial live in Fp2, and the
// low-degree test is the money-grade FRI with grinding. An honest trace verifies;
// any corrupted row breaks a transition and is rejected.
fn fib_trace(t: usize) -> alloc::vec::Vec<Fp> {
    let mut trace = alloc::vec![Fp::ONE, Fp::ONE];
    for i in 2..t {
        let next = trace[i - 1] + trace[i - 2];
        trace.push(next);
    }
    trace
}

#[test]
fn the_money_grade_stark_verifies_a_fibonacci_trace() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Fibonacci};
    let air = Fibonacci { log_t: 4 };
    let trace = fib_trace(1 << 4);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(stark_verify_ext(&air, &proof, 32, 8), "honest money-grade STARK rejected");
}

#[test]
fn a_tampered_money_grade_trace_is_rejected() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Fibonacci};
    let air = Fibonacci { log_t: 4 };
    let mut trace = fib_trace(1 << 4);
    trace[7] = trace[7] + Fp::ONE; // breaks the recurrence at a non-exempt row
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(!stark_verify_ext(&air, &proof, 32, 8), "a tampered money-grade trace verified");
}

// The value-conservation constraint, proven money-grade: a running-sum accumulator
// whose signed addends (inputs positive, outputs and fee negative) must cancel. A
// balanced set verifies; any imbalance (value created) breaks the end boundary and
// is rejected. This is the no-inflation gate, proven at ~2^-128 soundness.
fn neg(x: u64) -> Fp {
    Fp::ZERO - Fp::from_u64(x)
}

fn accumulator_trace(addends: &[Fp]) -> alloc::vec::Vec<Fp> {
    let mut trace = alloc::vec::Vec::with_capacity(addends.len() * 2);
    let mut acc = Fp::ZERO;
    for &a in addends {
        trace.push(acc); // column 0: running total
        trace.push(a); // column 1: addend
        acc = acc + a;
    }
    trace
}

#[test]
fn the_money_grade_stark_proves_value_conservation() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Accumulator};
    let air = Accumulator { log_t: 3 };
    // inputs 7, 3; outputs 8, 1; fee 1; padding 0, 0. First seven sum to zero.
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let trace = accumulator_trace(&addends);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(stark_verify_ext(&air, &proof, 32, 8), "a balanced (conserving) trace was rejected");
}

#[test]
fn the_money_grade_stark_rejects_inflation() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Accumulator};
    let air = Accumulator { log_t: 3 };
    // The addends no longer cancel: one extra unit of value is created.
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let trace = accumulator_trace(&addends);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(!stark_verify_ext(&air, &proof, 32, 8), "an inflating trace verified");
}

// Poseidon proven INSIDE the money-grade STARK: knowledge of a preimage that
// hashes to a public digest, at ~2^-128 soundness. This is the primitive private
// membership and nullifier derivation are built from (a Merkle path is a chain of
// these compressions). Honest preimage verifies; a forged digest is rejected.
#[test]
fn the_money_grade_stark_proves_a_poseidon_preimage() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let params = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let (trace, digest) = poseidon_trace(&params, absorb(sample_input()), POSEIDON_LOG_T);
    let air = Poseidon::new(POSEIDON_LOG_T, digest);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(stark_verify_ext(&air, &proof, 32, 8), "honest money-grade poseidon preimage rejected");
}

#[test]
fn a_money_grade_poseidon_forged_digest_is_rejected() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let params = Poseidon::new(POSEIDON_LOG_T, [Fp::ZERO; RATE]);
    let (trace, digest) = poseidon_trace(&params, absorb(sample_input()), POSEIDON_LOG_T);
    let mut wrong = digest;
    wrong[0] = wrong[0] + Fp::ONE;
    let air = Poseidon::new(POSEIDON_LOG_T, wrong);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(
        !stark_verify_ext(&air, &proof, 32, 8),
        "a forged digest verified in the money-grade STARK"
    );
}

// Private set membership proven at money-grade soundness: a leaf opens to a public
// Poseidon-Merkle root without the leaf appearing in the public statement. This is
// the core of the pool's membership check (and nullifier derivation is the same
// hash-chain shape). Honest opening verifies; a wrong root is rejected.
fn prove_membership_ext(
    hasher: &Poseidon,
    leaves: &[[Fp; RATE]],
    index: usize,
    log_rounds: u32,
) -> bool {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let tree = PoseidonMerkleTree::commit(hasher, leaves);
    let root = tree.root();
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, root, path, directions);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    stark_verify_ext(&air, &proof, 32, 8)
}

#[test]
fn the_money_grade_stark_proves_merkle_membership() {
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    assert!(
        prove_membership_ext(&hasher, &merkle_leaves(8), 5, log_rounds),
        "money-grade membership rejected"
    );
}

#[test]
fn a_money_grade_membership_for_a_wrong_root_is_rejected() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let index = 5usize;
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let mut wrong = tree.root();
    wrong[0] = wrong[0] + Fp::ONE;
    let air = MerkleMembership::new(hasher.clone(), log_rounds, wrong, path, directions);
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(
        !stark_verify_ext(&air, &proof, 32, 8),
        "a money-grade membership for a wrong root verified"
    );
}

// Range by bit decomposition, proven money-grade: the value peels into booleans and
// the remainder reaches zero, so it fits the bit width. An out-of-range value leaves
// a nonzero remainder and is rejected. This is the overflow guard on note values.
fn range_trace(value: u64, log_t: u32) -> alloc::vec::Vec<Fp> {
    let t = 1usize << log_t;
    let mut trace = alloc::vec::Vec::with_capacity(t * 2);
    let mut acc = value;
    for i in 0..t {
        let bit = if i < t - 1 { acc & 1 } else { 0 };
        trace.push(Fp::from_u64(acc));
        trace.push(Fp::from_u64(bit));
        if i < t - 1 {
            acc >>= 1;
        }
    }
    trace
}

#[test]
fn the_money_grade_stark_proves_a_value_in_range() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, RangeCheck};
    let air = RangeCheck { log_t: 4 }; // bound 2^15
    let proof = stark_prove_ext(&air, &range_trace(12345, 4), 32, 8);
    assert!(stark_verify_ext(&air, &proof, 32, 8), "an in-range value was rejected");
}

#[test]
fn the_money_grade_stark_rejects_an_out_of_range_value() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, RangeCheck};
    let air = RangeCheck { log_t: 4 };
    let proof = stark_prove_ext(&air, &range_trace(1u64 << 15, 4), 32, 8);
    assert!(!stark_verify_ext(&air, &proof, 32, 8), "an out-of-range value verified");
}

// The money-grade fusion: two independent constraint systems (value conservation
// AND range) proven as ONE STARK, each region's constraints firing under its
// selector. This is the composition shape of the full join-split proof; the honest
// compound trace verifies, and breaking either region is rejected.
#[test]
fn the_money_grade_stark_fuses_conservation_and_range() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Accumulator, AirExt, FusedExt, RangeCheck,
    };
    use alloc::boxed::Box;
    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 })
    ];
    let fused = FusedExt::new(regions);
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let cons = accumulator_trace(&addends);
    let rng = range_trace(12345, 4);
    let trace = fused.trace(&[cons, rng]);
    let proof = stark_prove_ext(&fused, &trace, 32, 8);
    assert!(stark_verify_ext(&fused, &proof, 32, 8), "an honest fused compound proof was rejected");
}

#[test]
fn the_fused_money_grade_stark_rejects_a_broken_region() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Accumulator, AirExt, FusedExt, RangeCheck,
    };
    use alloc::boxed::Box;
    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 })
    ];
    let fused = FusedExt::new(regions);
    // Conservation broken (addends do not cancel), range fine.
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let cons = accumulator_trace(&addends);
    let rng = range_trace(12345, 4);
    let trace = fused.trace(&[cons, rng]);
    let proof = stark_prove_ext(&fused, &trace, 32, 8);
    assert!(
        !stark_verify_ext(&fused, &proof, 32, 8),
        "a fused proof with a broken region verified"
    );
}

// The join-split CORE as ONE money-grade STARK: value conservation AND range AND a
// private membership opening, three real regions fused and proven together at
// ~2^-128. This is the compound shape the pool's settlement proves (nullifier and
// commitment are the same Poseidon primitive; copy-constraint wiring binds the
// shared value). Honest compound witness verifies.
#[test]
fn the_money_grade_stark_proves_the_join_split_core() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Accumulator, AirExt, FusedExt, MerkleMembership,
        RangeCheck,
    };
    use crate::crypto::stark::poseidon_merkle::PoseidonMerkleTree;
    use alloc::boxed::Box;

    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let index = 5usize;
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let root = tree.root();
    let path = tree.open(index);
    let directions: alloc::vec::Vec<bool> =
        (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let mem_trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let mem_air = MerkleMembership::new(hasher.clone(), log_rounds, root, path, directions);

    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
        Box::new(mem_air),
    ];
    let fused = FusedExt::new(regions);

    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let cons = accumulator_trace(&addends);
    let rng = range_trace(12345, 4);
    let trace = fused.trace(&[cons, rng, mem_trace]);

    let proof = stark_prove_ext(&fused, &trace, 32, 8);
    assert!(stark_verify_ext(&fused, &proof, 32, 8), "the join-split core proof was rejected");
}

// A money-grade WIRED binding: region A produces a value, region B starts from it,
// and a copy constraint forces A's last cell equal to B's first, so the two
// internally-valid regions must agree on the shared value -- all at ~2^-128. This
// is how the join-split binds one note's value across its conservation, range, and
// commitment regions. Honest binding verifies; a mismatched handoff is rejected.
#[test]
fn the_money_grade_stark_wires_a_shared_value() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, AirExt, Squaring, WiredExt,
    };
    use alloc::boxed::Box;
    let a_trace = squaring_trace(3, Fp::from_u64(3));
    let handoff = a_trace[7];
    let b_trace = squaring_trace(3, handoff);
    let mut sigma: alloc::vec::Vec<usize> = (0..16).collect();
    sigma.swap(7, 8); // A's last cell wired to B's first
    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Squaring { log_t: 3, seed: Fp::from_u64(3) }) as Box<dyn AirExt>,
        Box::new(Squaring { log_t: 3, seed: handoff }),
    ];
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[a_trace, b_trace]);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(stark_verify_ext(&wired, &proof, 32, 8), "an honest money-grade wiring was rejected");
}

#[test]
fn the_money_grade_stark_rejects_a_broken_wire() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, AirExt, Squaring, WiredExt,
    };
    use alloc::boxed::Box;
    let a_trace = squaring_trace(3, Fp::from_u64(3));
    let b_trace = squaring_trace(3, Fp::from_u64(99)); // B starts from a DIFFERENT value
    let mut sigma: alloc::vec::Vec<usize> = (0..16).collect();
    sigma.swap(7, 8);
    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Squaring { log_t: 3, seed: Fp::from_u64(3) }) as Box<dyn AirExt>,
        Box::new(Squaring { log_t: 3, seed: Fp::from_u64(99) }),
    ];
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[a_trace, b_trace]);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(!stark_verify_ext(&wired, &proof, 32, 8), "a broken money-grade wire verified");
}

// A WIRED join-split at money-grade: value conservation AND range on the value AND
// a copy constraint binding conservation's input to the range-checked value, so
// they must be the SAME note value (not two independent statements). This is the
// real join-split shape. Honest verifies; unbinding the value is rejected.
#[test]
fn the_money_grade_stark_proves_a_wired_join_split() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Accumulator, AirExt, RangeCheck, WiredExt,
    };
    use alloc::boxed::Box;
    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    // span = (8 + 16).next_pow2() = 32. Wire col 0: conservation acc[1] (fused row 1,
    // = the input 7) bound to range acc[0] (fused row 8, the range-checked value).
    let mut sigma: alloc::vec::Vec<usize> = (0..32).collect();
    sigma.swap(1, 8);
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let cons = accumulator_trace(&addends); // acc[1] = 7
    let rng = range_trace(7, 4); // range-checks 7
    let witness = wired.trace(&[cons, rng]);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(stark_verify_ext(&wired, &proof, 32, 8), "an honest wired join-split was rejected");
}

#[test]
fn the_wired_join_split_rejects_an_unbound_value() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Accumulator, AirExt, RangeCheck, WiredExt,
    };
    use alloc::boxed::Box;
    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: alloc::vec::Vec<usize> = (0..32).collect();
    sigma.swap(1, 8);
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let cons = accumulator_trace(&addends); // acc[1] = 7
    let rng = range_trace(9999, 4); // range-checks a DIFFERENT value
    let witness = wired.trace(&[cons, rng]);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(!stark_verify_ext(&wired, &proof, 32, 8), "an unbound wired join-split verified");
}

// The FRI verification arithmetized as ONE money-grade STARK: per query a Merkle
// opening region and a fold region, wired so each opening's leaf equals its fold's
// opened value and the layer challenges agree across queries. Proven at ~2^-128.
// This is the recursion primitive -- a proof's own verification, provable in a
// proof -- and it reuses the base trace helpers unchanged, only WiredExt + AirExt.
fn whole_proof_monolith_ext(
    queries: &[usize],
    seeds: &[u64],
) -> (crate::crypto::stark::air::WiredExt, Vec<Fp>) {
    use crate::crypto::stark::air::{AirExt, TraceFold, WiredExt};
    use alloc::boxed::Box;
    let mut regions: Vec<Box<dyn AirExt>> = Vec::new();
    let mut traces: Vec<Vec<Fp>> = Vec::new();
    let mut heights: Vec<usize> = Vec::new();
    let mut open_idx: Vec<usize> = Vec::new();
    let mut fold_idx: Vec<usize> = Vec::new();
    let mut n_folds = 0usize;

    for (&query, &seed) in queries.iter().zip(seeds) {
        let (beta, a, b, x_inv, dir, fv, ll, nf) = trace_fold_data_seeded(query, seed);
        n_folds = nf;
        let (mtr, mem, _) = opening_of_scalar(a[0], 2);
        open_idx.push(regions.len());
        heights.push(mem.rows());
        traces.push(mtr);
        regions.push(Box::new(mem));

        let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
        fold_idx.push(regions.len());
        heights.push(1usize << ll);
        traces.push(fold.trace(&beta, &a, &b));
        regions.push(Box::new(fold));
    }

    let mut offsets: Vec<usize> = Vec::new();
    let mut acc = 0usize;
    for &h in &heights {
        offsets.push(acc);
        acc += h;
    }
    let span = acc.next_power_of_two();
    let k = 2usize;
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for qi in 0..queries.len() {
        let leaf = offsets[open_idx[qi]] * k;
        let opened = offsets[fold_idx[qi]] * k + 1;
        sigma.swap(leaf, opened);
    }
    let qn = queries.len();
    for m in 0..n_folds {
        for qi in 0..qn {
            let here = (offsets[fold_idx[qi]] + m) * k;
            let next = (offsets[fold_idx[(qi + 1) % qn]] + m) * k;
            sigma[here] = next;
        }
    }
    let wired = WiredExt::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&traces);
    (wired, witness)
}

#[test]
fn the_recursive_fri_verifier_is_one_money_grade_stark() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let seed = 0xf01d_1234u64 | 1;
    let (wired, witness) = whole_proof_monolith_ext(&[6, 10], &[seed, seed]);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &proof, 32, 8),
        "the money-grade recursion monolith was rejected"
    );
}

// Fiat-Shamir challenge derivation, arithmetized and proven money-grade: a
// challenge squeezed from a Poseidon transcript, verified at ~2^-128. This is the
// last recursion building block -- a recursive verifier runs its own transcript in
// circuit -- and it confirms every recursion primitive (transcript, FRI fold,
// Merkle opening, wiring) is now money-grade. Honest challenge verifies; wrong one
// is rejected.
#[test]
fn money_grade_fiat_shamir_challenge_derivation() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, FiatShamir};
    let (log_rounds, log_slots) = (3u32, 2u32);
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let inputs = alloc::vec![Fp::from_u64(111), Fp::from_u64(222), Fp::from_u64(333)];
    let (trace, challenge) = fiat_shamir_trace(&hasher, &inputs, log_rounds, log_slots);
    let air = FiatShamir::new(
        Poseidon::new(log_rounds, [Fp::ZERO; RATE]),
        log_rounds,
        log_slots,
        inputs.clone(),
        challenge,
    );
    let proof = stark_prove_ext(&air, &trace, 32, 8);
    assert!(stark_verify_ext(&air, &proof, 32, 8), "an honest money-grade transcript was rejected");

    let bad = FiatShamir::new(
        Poseidon::new(log_rounds, [Fp::ZERO; RATE]),
        log_rounds,
        log_slots,
        inputs,
        challenge + Fp::ONE,
    );
    let bad_proof = stark_prove_ext(&bad, &trace, 32, 8);
    assert!(!stark_verify_ext(&bad, &bad_proof, 32, 8), "a wrong money-grade challenge verified");
}

// The DEEP-consistency check, arithmetized and proven money-grade: a query's DEEP
// value must be the coefficient combination of the honestly-formed quotients of the
// opened trace and composition against the out-of-domain claims. This is the last
// verifier stage a recursive proof needs. Honest verifies; a wrong DEEP value is
// rejected (its quotient-combination no longer matches the pinned value).
#[test]
fn the_money_grade_stark_proves_deep_consistency() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, DeepCheck};
    let (tv, cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(5),
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    let q = (tv - cl) * xz_inv;
    let qc = (cp - cpz) * xz_inv;
    let deep = c0 * q + e * qc;
    let air = DeepCheck { trace_val: tv, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };
    let proof = stark_prove_ext(&air, &air.trace(), 32, 8);
    assert!(stark_verify_ext(&air, &proof, 32, 8), "honest DEEP consistency was rejected");
}

#[test]
fn the_money_grade_stark_rejects_a_wrong_deep_value() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, DeepCheck};
    let (tv, cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(5),
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    let q = (tv - cl) * xz_inv;
    let qc = (cp - cpz) * xz_inv;
    let deep = c0 * q + e * qc + Fp::ONE; // wrong DEEP value
    let air = DeepCheck { trace_val: tv, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };
    let proof = stark_prove_ext(&air, &air.trace(), 32, 8);
    assert!(!stark_verify_ext(&air, &proof, 32, 8), "a wrong DEEP value verified");
}

// The full recursive verifier as ONE money-grade STARK: all four verification
// stages -- Fiat-Shamir transcript derivation, FRI fold, Merkle opening, and DEEP
// consistency -- fused and proven together at ~2^-128. This is a proof's entire
// verification, provable in a proof. Wiring the stages' shared values is the
// soundness refinement (the wired join-split shows that shape); here the stages
// compose into one money-grade proof.
#[test]
fn the_full_recursive_verifier_is_one_money_grade_stark() {
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, AirExt, DeepCheck, FiatShamir, FusedExt, TraceFold,
    };
    use alloc::boxed::Box;

    // Stage 1: transcript derivation.
    let (lr, ls) = (3u32, 2u32);
    let hasher = Poseidon::new(lr, [Fp::ZERO; RATE]);
    let inputs = alloc::vec![Fp::from_u64(111), Fp::from_u64(222), Fp::from_u64(333)];
    let (fs_trace, challenge) = fiat_shamir_trace(&hasher, &inputs, lr, ls);
    let fs = FiatShamir::new(Poseidon::new(lr, [Fp::ZERO; RATE]), lr, ls, inputs, challenge);

    // Stage 2 + 3: a FRI fold and the Merkle opening of the folded value.
    let (beta, a, b, x_inv, dir, fv, ll, nf) = trace_fold_data_seeded(6, 0xf01d_1234u64 | 1);
    let (mtr, mem, _) = opening_of_scalar(a[0], 2);
    let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
    let fold_trace = fold.trace(&beta, &a, &b);

    // Stage 4: DEEP consistency.
    let (tv, cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(5),
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    let deep = c0 * ((tv - cl) * xz_inv) + e * ((cp - cpz) * xz_inv);
    let dc = DeepCheck { trace_val: tv, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };

    let regions: alloc::vec::Vec<Box<dyn AirExt>> =
        alloc::vec![Box::new(fs) as Box<dyn AirExt>, Box::new(mem), Box::new(fold), Box::new(dc),];
    let fused = FusedExt::new(regions);
    let witness = fused.trace(&[
        fs_trace,
        mtr,
        fold_trace,
        DeepCheck { trace_val: tv, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e }.trace(),
    ]);
    let proof = stark_prove_ext(&fused, &witness, 32, 8);
    assert!(stark_verify_ext(&fused, &proof, 32, 8), "the full recursive verifier was rejected");
}

#[test]
#[ignore]
fn gen_recursive_selftest() {
    // Emit the full recursive-verifier proof: the four verification stages
    // (Fiat-Shamir, FRI fold, Merkle opening, DEEP consistency) proven together as
    // one money-grade STARK. This is the recursive-verifier vector the pool's
    // constant-gas StarkVerifier is built against; its _composeConstraints is the
    // fused sum of the four stage transitions (transcribe from the AIR sources).
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, AirExt, DeepCheck, FiatShamir, FusedExt, TraceFold,
    };
    use alloc::boxed::Box;

    let (lr, ls) = (3u32, 2u32);
    let hasher = Poseidon::new(lr, [Fp::ZERO; RATE]);
    let inputs = alloc::vec![Fp::from_u64(111), Fp::from_u64(222), Fp::from_u64(333)];
    let (fs_trace, challenge) = fiat_shamir_trace(&hasher, &inputs, lr, ls);
    let fs = FiatShamir::new(Poseidon::new(lr, [Fp::ZERO; RATE]), lr, ls, inputs, challenge);

    let (beta, a, b, x_inv, dir, fv, ll, nf) = trace_fold_data_seeded(6, 0xf01d_1234u64 | 1);
    let (mtr, mem, _) = opening_of_scalar(a[0], 2);
    let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
    let fold_trace = fold.trace(&beta, &a, &b);

    let (tv, cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(5),
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    let deep = c0 * ((tv - cl) * xz_inv) + e * ((cp - cpz) * xz_inv);
    let dc = || DeepCheck { trace_val: tv, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };

    let regions: alloc::vec::Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(fs) as Box<dyn AirExt>,
        Box::new(mem),
        Box::new(fold),
        Box::new(dc()),
    ];
    let fused = FusedExt::new(regions);
    let witness = fused.trace(&[fs_trace, mtr, fold_trace, dc().trace()]);
    let proof = stark_prove_ext(&fused, &witness, 32, 8);
    assert!(stark_verify_ext(&fused, &proof, 32, 8), "recursive self-test does not verify");

    let bytes = crate::stark_selftest_gen::serialize(&proof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"air\": \"recursive-verifier (fiat-shamir + fri-fold + merkle-opening + deep-consistency)\",\n  \"note\": \"The full STARK verification arithmetized as one money-grade proof -- the CONSTANT-GAS target. _composeConstraints = the fused sum of the four stage transitions. Cross-stage sigma wiring is the soundness refinement (see the wired join-split shape).\",\n  \"params\": {{ \"n_queries\": 32, \"grind_bits\": 8 }},\n  \"stages\": [\"fiat_shamir\", \"merkle_membership\", \"trace_fold\", \"deep_check\"],\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        bytes.len(), crate::stark_selftest_gen::hex(&bytes)
    );
    std::fs::write("/Users/ek/Desktop/NOX-SmartContract/spec/recursive-selftest.json", &json)
        .expect("write recursive self-test");
    std::println!("wrote {} proof bytes to recursive-selftest.json", bytes.len());
}

// The cross-stage wiring that makes the recursive verifier SOUND: the value a
// Merkle opening reveals is bound by a copy constraint to the value the DEEP check
// consumes, so the two stages cannot be about different values. Proven money-grade;
// a mismatched value is rejected. This is the wiring the full recursive verifier
// uses to bind transcript->FRI->DEEP->Merkle; here it binds Merkle->DEEP.
fn wired_recursive_check(deep_val: Fp) -> (crate::crypto::stark::air::WiredExt, Vec<Fp>) {
    use crate::crypto::stark::air::{AirExt, DeepCheck, WiredExt};
    use alloc::boxed::Box;
    let scalar = Fp::from_u64(777);
    let (mtr, mem, _) = opening_of_scalar(scalar, 2);
    let mem_height = mem.rows();

    let (cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    // Honest DEEP value is the combination for THIS trace_val (deep_val).
    let deep = c0 * ((deep_val - cl) * xz_inv) + e * ((cp - cpz) * xz_inv);
    let dc =
        DeepCheck { trace_val: deep_val, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };
    let dc_trace = dc.trace();

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![Box::new(mem) as Box<dyn AirExt>, Box::new(dc)];
    let span = (mem_height + 2).next_power_of_two();
    let mut sigma: Vec<usize> = (0..span).collect();
    // Merkle leaf (row 0, col 0) <-> DEEP trace_val (row mem_height, col 0).
    sigma.swap(0, mem_height);
    let wired = WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[mtr, dc_trace]);
    (wired, witness)
}

#[test]
fn the_wired_recursive_verifier_binds_stages() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    // The DEEP value uses the SAME value the Merkle opening revealed (777).
    let (wired, witness) = wired_recursive_check(Fp::from_u64(777));
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &proof, 32, 8),
        "an honestly-bound recursive verifier was rejected"
    );
}

#[test]
fn the_wired_recursive_verifier_rejects_a_stage_mismatch() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    // The DEEP check uses a DIFFERENT value (888) than the Merkle opening (777):
    // the wire between stages breaks.
    let (wired, witness) = wired_recursive_check(Fp::from_u64(888));
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        !stark_verify_ext(&wired, &proof, 32, 8),
        "a stage-mismatched recursive verifier verified"
    );
}

#[test]
#[ignore]
fn gen_recursive_public_selftest() {
    // The recursive-verifier vector WITH its public statement: the proof plus every
    // boundary triple and every periodic column, so the verifier can reconstruct
    // periodic_z and compose_ext and actually run. Poseidon round constants are
    // regenerable from the schedule, but included here so the vector is fully
    // self-contained and testable.
    use crate::crypto::stark::air::{
        stark_prove_ext, stark_verify_ext, Air, AirExt, DeepCheck, FiatShamir, FusedExt, TraceFold,
    };
    use alloc::boxed::Box;
    use alloc::string::String;

    let (lr, ls) = (3u32, 2u32);
    let hasher = Poseidon::new(lr, [Fp::ZERO; RATE]);
    let inputs = alloc::vec![Fp::from_u64(111), Fp::from_u64(222), Fp::from_u64(333)];
    let (fs_trace, challenge) = fiat_shamir_trace(&hasher, &inputs, lr, ls);
    let fs = FiatShamir::new(Poseidon::new(lr, [Fp::ZERO; RATE]), lr, ls, inputs, challenge);
    let (beta, a, b, x_inv, dir, fv, ll, nf) = trace_fold_data_seeded(6, 0xf01d_1234u64 | 1);
    let (mtr, mem, _) = opening_of_scalar(a[0], 2);
    let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
    let fold_trace = fold.trace(&beta, &a, &b);
    let (tv, cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(5),
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    let deep = c0 * ((tv - cl) * xz_inv) + e * ((cp - cpz) * xz_inv);
    let dc = || DeepCheck { trace_val: tv, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };

    let regions: alloc::vec::Vec<Box<dyn AirExt>> =
        alloc::vec![Box::new(fs) as Box<dyn AirExt>, Box::new(mem), Box::new(fold), Box::new(dc())];
    let fused = FusedExt::new(regions);
    let witness = fused.trace(&[fs_trace, mtr, fold_trace, dc().trace()]);
    let proof = stark_prove_ext(&fused, &witness, 32, 8);
    assert!(stark_verify_ext(&fused, &proof, 32, 8), "recursive public self-test does not verify");

    // Public statement: boundaries + periodic columns.
    let mut bnd = String::from("[");
    for (i, (c, r, v)) in fused.boundary().iter().enumerate() {
        if i > 0 {
            bnd.push(',');
        }
        bnd.push_str(&alloc::format!("[{},{},\"{}\"]", c, r, v.value()));
    }
    bnd.push(']');
    let mut per = String::from("[");
    for (i, col) in fused.periodic_columns().iter().enumerate() {
        if i > 0 {
            per.push(',');
        }
        per.push('[');
        for (j, v) in col.iter().enumerate() {
            if j > 0 {
                per.push(',');
            }
            per.push_str(&alloc::format!("\"{}\"", v.value()));
        }
        per.push(']');
    }
    per.push(']');

    let bytes = crate::stark_selftest_gen::serialize(&proof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"air\": \"recursive-verifier (fiat-shamir + fri-fold + merkle-opening + deep-consistency)\",\n  \"note\": \"Full recursive verification with its PUBLIC STATEMENT. boundaries = (col,row,value) pins; periodic_columns = every periodic column expanded (selectors, per-stage instance data, Poseidon RCs). The verifier reconstructs periodic_z via eval_lagrange_ext and runs compose_ext. This is the FusedExt composition; cross-stage sigma wiring is the soundness refinement.\",\n  \"log_trace_len\": {}, \"trace_width\": {}, \"n_queries\": 32, \"grind_bits\": 8,\n  \"stages\": [\"fiat_shamir\", \"merkle_membership\", \"trace_fold\", \"deep_check\"],\n  \"boundaries\": {},\n  \"periodic_columns\": {},\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        fused.log_trace_len(), fused.trace_width(), bnd, per, bytes.len(), crate::stark_selftest_gen::hex(&bytes)
    );
    std::fs::write("/Users/ek/Desktop/NOX-SmartContract/spec/recursive-selftest.json", &json)
        .expect("write");
    std::println!(
        "wrote proof {} bytes + {} boundaries + {} periodic cols",
        bytes.len(),
        fused.boundary().len(),
        fused.periodic_columns().len()
    );
}

// How to fault the wired recursive verifier, so each wire is shown load-bearing.
enum RecursiveFault {
    None,
    // The DEEP check is about a value the opening never committed.
    RebindValue(Fp),
    // The fold runs on a challenge the transcript never squeezed.
    UnboundChallenge(Fp),
}

// The fully wired 4-stage recursive verifier: the same four stages the fused vector
// composes (Fiat-Shamir, Merkle opening, FRI fold, DEEP consistency), now bound end
// to end by one grand-product column carrying two cross-stage cycles:
//   value flow: Merkle-opened value == fold input == DEEP trace value.
//   transcript: Fiat-Shamir challenge == the fold's first-layer beta.
// So the four stages are provably about one value and driven by one challenge; a
// verifier can neither fold or DEEP-check a value the opening never revealed, nor
// fold on a challenge the transcript never squeezed. A faulted wire breaks the
// product without touching any region's own constraint. This is the custody-flip
// shape the pool's constant-gas verifier points at.
fn wired_recursive_verifier(
    fault: RecursiveFault,
) -> (crate::crypto::stark::air::WiredExt, Vec<Fp>) {
    use crate::crypto::stark::air::{AirExt, DeepCheck, FiatShamir, TraceFold, WiredExt};
    use alloc::boxed::Box;

    // Stage 1: transcript derivation.
    let (lr, ls) = (3u32, 2u32);
    let hasher = Poseidon::new(lr, [Fp::ZERO; RATE]);
    let inputs = alloc::vec![Fp::from_u64(111), Fp::from_u64(222), Fp::from_u64(333)];
    let (fs_trace, challenge) = fiat_shamir_trace(&hasher, &inputs, lr, ls);
    let fs = FiatShamir::new(Poseidon::new(lr, [Fp::ZERO; RATE]), lr, ls, inputs, challenge);

    // Stage 2 + 3: the Merkle opening of a[0], and the fold that consumes a[0]. The
    // fold's first beta is the transcript challenge, unless faulted off-transcript.
    let fold_beta0 = match fault {
        RecursiveFault::UnboundChallenge(b) => b,
        _ => challenge,
    };
    let (beta, a, b, x_inv, dir, fv, ll, nf) =
        trace_fold_data_seeded_first(6, 0xf01d_1234u64 | 1, Some(fold_beta0));
    let (mtr, mem, _) = opening_of_scalar(a[0], 2);
    let fold = TraceFold::new(ll, nf, x_inv, dir, fv);
    let fold_trace = fold.trace(&beta, &a, &b);

    // Stage 4: DEEP consistency, honestly formed for whatever value it is about.
    let deep_val = match fault {
        RecursiveFault::RebindValue(v) => v,
        _ => a[0],
    };
    let (cl, cp, cpz, x, z, c0, e) = (
        Fp::from_u64(2),
        Fp::from_u64(8),
        Fp::from_u64(1),
        Fp::from_u64(10),
        Fp::from_u64(3),
        Fp::from_u64(4),
        Fp::from_u64(6),
    );
    let xz_inv = (x - z).inv();
    let deep = c0 * ((deep_val - cl) * xz_inv) + e * ((cp - cpz) * xz_inv);
    let dc =
        DeepCheck { trace_val: deep_val, claimed: cl, comp: cp, comp_z: cpz, deep, x, z, c0, e };
    let dc_trace = dc.trace();

    let regions: Vec<Box<dyn AirExt>> =
        alloc::vec![Box::new(fs) as Box<dyn AirExt>, Box::new(mem), Box::new(fold), Box::new(dc),];

    // Region row offsets exactly as the engine stacks them: each region takes
    // its actual rows, unpadded. The FS challenge sits at row (2^ls - 1) * 2^lr,
    // column 0 of its own region.
    let mut offs = Vec::with_capacity(regions.len());
    let mut row = 0usize;
    for r in &regions {
        offs.push(row);
        row += r.rows();
    }
    let span = row.next_power_of_two();
    let (o_mem, o_fold, o_dc) = (offs[1], offs[2], offs[3]);
    let fs_challenge_row = ((1usize << ls) - 1) * (1usize << lr);

    // wired columns 0 and 1, so k = 2 and a cell's id is row*2 + wired_index.
    let k = 2usize;
    let mut sigma: Vec<usize> = (0..span * k).collect();
    // Value-flow 3-cycle: Merkle leaf (col 0) -> fold input (col 1) -> DEEP (col 0).
    let (id_mem, id_fold_a, id_dc) = (o_mem * k, o_fold * k + 1, o_dc * k);
    sigma[id_mem] = id_fold_a;
    sigma[id_fold_a] = id_dc;
    sigma[id_dc] = id_mem;
    // Transcript 2-cycle: FS challenge (col 0) <-> fold first beta (col 0).
    sigma.swap(fs_challenge_row * k, o_fold * k);

    let wired = WiredExt::new(regions, alloc::vec![0, 1], sigma, Fp::from_u64(5), Fp::from_u64(7));
    let witness = wired.trace(&[fs_trace, mtr, fold_trace, dc_trace]);
    (wired, witness)
}

// The honestly wired recursive verifier: opened == folded == DEEP-checked value, and
// the fold runs on the transcript challenge. Both cross-stage cycles telescope, and
// the proof verifies at ~2^-128.
#[test]
fn the_full_wired_recursive_verifier_binds_the_value_flow() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let (wired, witness) = wired_recursive_verifier(RecursiveFault::None);
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(
        stark_verify_ext(&wired, &proof, 32, 8),
        "the honestly wired recursive verifier was rejected"
    );
}

// The DEEP check is about a value the Merkle stage never opened. Its own constraint
// still holds, but the value-flow wire breaks, so the grand product no longer
// returns to one and the proof is rejected.
#[test]
fn the_full_wired_recursive_verifier_rejects_a_rebound_value() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let (wired, witness) =
        wired_recursive_verifier(RecursiveFault::RebindValue(Fp::from_u64(0xD1FF)));
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(!stark_verify_ext(&wired, &proof, 32, 8), "a rebound recursive verifier verified");
}

// The fold runs on a challenge the transcript never squeezed. The fold is internally
// consistent for that challenge, but the transcript wire breaks, so the proof is
// rejected. This is what forces the recursive verifier to be honest about Fiat-Shamir.
#[test]
fn the_full_wired_recursive_verifier_rejects_an_off_transcript_fold() {
    use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
    let (wired, witness) =
        wired_recursive_verifier(RecursiveFault::UnboundChallenge(Fp::from_u64(0xBAD0)));
    let proof = stark_prove_ext(&wired, &witness, 32, 8);
    assert!(!stark_verify_ext(&wired, &proof, 32, 8), "an off-transcript fold verified");
}

#[test]
#[ignore]
fn gen_wired_recursive_public_selftest() {
    // The WIRED recursive-verifier vector with its public statement: the same four
    // stages as the fused vector, plus the one grand-product column that binds the
    // opened value to the folded and DEEP-checked value AND the transcript challenge
    // to the fold's beta. The verifier gains exactly one product term in its compose
    // and one boundary pinning that column to one; everything else (transcript, FRI,
    // Merkle, Fp2, periodic eval) is unchanged. This is the custody-flip vector.
    use crate::crypto::stark::air::{stark_prove_ext_blown, stark_verify_ext_blown, Air};
    use alloc::string::String;

    // Deployment soundness. The FRI runs at rate 1/2 by default (1 conjectured bit
    // per query), so the 32-query test instances are only ~40-bit. For a fund gate
    // the vector is generated at rate 1/16 (EXTRA_BLOWUP_BITS = 3, 4 conjectured
    // bits per query): 32 queries give 128 bits and 16 grind bits add margin.
    use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, N_QUERIES};

    let (wired, witness) = wired_recursive_verifier(RecursiveFault::None);
    let proof = stark_prove_ext_blown(&wired, &witness, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS);
    assert!(
        stark_verify_ext_blown(&wired, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS),
        "wired recursive deployment self-test does not verify"
    );

    // The wiring must still reject at the deployment parameters, not just in the
    // fast tests: a value the opening never committed breaks the grand product.
    let (bad, bad_w) = wired_recursive_verifier(RecursiveFault::RebindValue(Fp::from_u64(0xD1FF)));
    let bad_proof = stark_prove_ext_blown(&bad, &bad_w, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS);
    assert!(
        !stark_verify_ext_blown(&bad, &bad_proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS),
        "a rebound value verified at deployment parameters"
    );

    let mut bnd = String::from("[");
    for (i, (c, r, v)) in wired.boundary().iter().enumerate() {
        if i > 0 {
            bnd.push(',');
        }
        bnd.push_str(&alloc::format!("[{},{},\"{}\"]", c, r, v.value()));
    }
    bnd.push(']');
    let mut per = String::from("[");
    for (i, col) in wired.periodic_columns().iter().enumerate() {
        if i > 0 {
            per.push(',');
        }
        per.push('[');
        for (j, v) in col.iter().enumerate() {
            if j > 0 {
                per.push(',');
            }
            per.push_str(&alloc::format!("\"{}\"", v.value()));
        }
        per.push(']');
    }
    per.push(']');

    let bytes = crate::stark_selftest_gen::serialize(&proof);
    let json = alloc::format!(
        "{{\n  \"engine\": \"nonos-money-grade-stark\",\n  \"air\": \"wired-recursive-verifier (fiat-shamir + merkle-opening + fri-fold + deep-consistency, fully bound)\",\n  \"note\": \"The WIRED recursive verification with its PUBLIC STATEMENT, at DEPLOYMENT soundness. Adds one grand-product column to the fused composition carrying two cross-stage cycles: a value-flow cycle binds the Merkle-opened value to the fold input and the DEEP trace value, and a transcript cycle binds the Fiat-Shamir challenge to the fold's first beta. So the four stages are provably about one value and driven by one challenge. The FRI runs at rate 1/16 (extra_blowup_bits=3, fri_log_blowup=4), so 32 queries give 128 conjectured bits and 16 grind bits add margin. _composeConstraints = the fused sum of the four stage transitions PLUS the one grand-product term; boundaries include the product column pinned to one at row 0 and row span.\",\n  \"wiring\": {{ \"wired_cols\": [0, 1], \"beta\": 5, \"gamma\": 7 }},\n  \"soundness\": {{ \"extra_blowup_bits\": 3, \"fri_log_blowup\": 4, \"conjectured_bits\": 144, \"regime\": \"proximity-gap-conjectured\" }},\n  \"log_trace_len\": {}, \"trace_width\": {}, \"n_queries\": 32, \"grind_bits\": 16,\n  \"stages\": [\"fiat_shamir\", \"merkle_membership\", \"trace_fold\", \"deep_check\", \"grand_product\"],\n  \"boundaries\": {},\n  \"periodic_columns\": {},\n  \"proof_len_bytes\": {},\n  \"proof_hex\": \"{}\"\n}}\n",
        wired.log_trace_len(), wired.trace_width(), bnd, per, bytes.len(), crate::stark_selftest_gen::hex(&bytes)
    );
    std::fs::write("/Users/ek/Desktop/NOX-SmartContract/spec/wired-recursive-selftest.json", &json)
        .expect("write");
    std::println!(
        "wrote wired proof {} bytes + {} boundaries + {} periodic cols",
        bytes.len(),
        wired.boundary().len(),
        wired.periodic_columns().len()
    );
}

// A hostile capsule can put any 32-bit count in the trailer before the data
// that would back it. The deserializer must refuse such a trailer without first
// reserving gigabytes for a vector it will never fill. This builds a trailer
// that reaches the first length field and sets it to its maximum; the parser
// must return None promptly, which it cannot do if it pre-allocates the count.
#[test]
fn a_hostile_length_prefix_is_refused_without_overallocating() {
    use crate::crypto::stark::air::deserialize_proof_ext;
    let mut trailer = Vec::new();
    trailer.extend_from_slice(&[0u8; 32]); // trace_root
    trailer.extend_from_slice(&[0u8; 32]); // comp_root
    trailer.extend_from_slice(&0u32.to_le_bytes()); // ood frame: zero elements
    trailer.extend_from_slice(&u32::MAX.to_le_bytes()); // FRI roots: 2^32 - 1
    assert!(
        deserialize_proof_ext(&trailer).is_none(),
        "a trailer with a hostile length prefix must be refused"
    );
}
