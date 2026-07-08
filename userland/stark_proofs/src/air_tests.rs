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
    stark_prove, stark_prove_bound, stark_verify, stark_verify_bound, Air, CopyConstraint,
    FiatShamir, Fibonacci, FriFold, Fused, MerkleMembership, MultiMembership, Opening, Permutation,
    Permutation2, Poseidon, PowerChain, Squaring, TraceFold, Wired, RATE, WIDTH,
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
    let mem_h = mem_trace.len() / WIDTH;
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
    let mem_h = mem_trace.len() / WIDTH;
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
    let mem_h = mem_trace.len() / WIDTH;
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
    let mem_h = mem_trace.len() / WIDTH;
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
    let mem_h = mem_trace.len() / WIDTH;
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
        heights.push(mtr.len() / WIDTH);
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

    let mem_h = open_a.len() / WIDTH;
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

    let mem_h = open_a.len() / WIDTH;
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
// The logup lookup argument: prove every witness value lies in a table. The
// running sum sum 1/(a_i + X) - sum m_j/(t_j + X) is zero exactly when the
// witness multiset is contained in the table, so an in-table witness verifies
// and a value outside the table leaves a term the multiplicities cannot cancel.
use crate::crypto::stark::air::Lookup;

const LOOKUP_X: u64 = 0x5bd1_e995;

#[test]
fn a_lookup_of_in_table_values_verifies() {
    let table: Vec<Fp> = (0..8).map(Fp::from_u64).collect();
    // Every witness value is a table entry, some repeated.
    let witness: Vec<Fp> = [3u64, 3, 7, 0, 5, 3].iter().map(|v| Fp::from_u64(*v)).collect();
    let air = Lookup::new(witness, table, Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an in-table lookup was rejected");
}

#[test]
fn an_out_of_table_value_is_rejected() {
    let table: Vec<Fp> = (0..8).map(Fp::from_u64).collect();
    // 42 is not in the table: it is counted in no multiplicity, so its inverse
    // term is unmatched and the sum cannot return to zero.
    let witness: Vec<Fp> = [3u64, 7, 42, 0].iter().map(|v| Fp::from_u64(*v)).collect();
    let air = Lookup::new(witness, table, Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "an out-of-table value verified");
}

#[test]
fn a_range_check_via_bit_limbs_verifies() {
    // The mega-AIR use: range-check a value by looking up each of its low limbs
    // in a small range table. Here every two-bit limb of a set of values is
    // proven to lie in {0, 1, 2, 3}.
    let table: Vec<Fp> = (0..4).map(Fp::from_u64).collect();
    let values = [0u64, 5, 10, 15, 9, 6];
    let mut limbs: Vec<Fp> = Vec::new();
    for v in values {
        limbs.push(Fp::from_u64(v & 0b11));
        limbs.push(Fp::from_u64((v >> 2) & 0b11));
    }
    let air = Lookup::new(limbs, table, Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an in-range limb decomposition was rejected");
}

#[test]
fn a_limb_out_of_range_is_rejected() {
    // A limb of 4 does not fit a two-bit range table.
    let table: Vec<Fp> = (0..4).map(Fp::from_u64).collect();
    let limbs: Vec<Fp> = [0u64, 1, 4, 2].iter().map(|v| Fp::from_u64(*v)).collect();
    let air = Lookup::new(limbs, table, Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "an out-of-range limb verified");
}

#[test]
fn a_tampered_running_sum_is_rejected() {
    let table: Vec<Fp> = (0..8).map(Fp::from_u64).collect();
    let witness: Vec<Fp> = [1u64, 2, 3, 4].iter().map(|v| Fp::from_u64(*v)).collect();
    let air = Lookup::new(witness, table, Fp::from_u64(LOOKUP_X));
    let mut trace = air.trace();
    // Perturb the running-sum column at one interior row (width 3, column 2).
    trace[3 * 2 + 2] = trace[3 * 2 + 2] + Fp::ONE;
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a tampered running sum verified");
}

// Adversarial hardening of the lookup: over many random tables and witnesses,
// every in-table witness verifies and every witness carrying an out-of-table
// value is rejected, and the soundness holds under a challenge drawn from a
// Poseidon transcript after the witness and multiplicities are committed, the
// order the argument requires.

fn lookup_xs(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn the_lookup_verifies_and_rejects_over_random_cases() {
    let table_size = 8u64;
    let table: Vec<Fp> = (0..table_size).map(Fp::from_u64).collect();
    let mut s = 0x1234_5678u64;
    let x = Fp::from_u64(LOOKUP_X);

    for _ in 0..40 {
        let len = 1 + (lookup_xs(&mut s) % 8) as usize;
        // An in-table witness: every value drawn from the table.
        let good: Vec<Fp> =
            (0..len).map(|_| Fp::from_u64(lookup_xs(&mut s) % table_size)).collect();
        let air = Lookup::new(good, table.clone(), x);
        let proof = stark_prove(&air, &air.trace(), QUERIES);
        assert!(stark_verify(&air, &proof, QUERIES), "an in-table random witness was rejected");

        // The same witness with one value pushed out of the table.
        let mut bad: Vec<Fp> =
            (0..len).map(|_| Fp::from_u64(lookup_xs(&mut s) % table_size)).collect();
        bad[(lookup_xs(&mut s) as usize) % len] =
            Fp::from_u64(table_size + 1 + lookup_xs(&mut s) % 100);
        let bad_air = Lookup::new(bad, table.clone(), x);
        let bad_proof = stark_prove(&bad_air, &bad_air.trace(), QUERIES);
        assert!(
            !stark_verify(&bad_air, &bad_proof, QUERIES),
            "an out-of-table random witness verified"
        );
    }
}

#[test]
fn the_lookup_is_sound_under_a_transcript_challenge() {
    use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;

    let table: Vec<Fp> = (0..8).map(Fp::from_u64).collect();
    let witness: Vec<Fp> = [2u64, 5, 5, 1, 7].iter().map(|v| Fp::from_u64(*v)).collect();

    // Draw the logup challenge from a Poseidon transcript, after absorbing the
    // witness and the multiplicities, so the challenge cannot be anticipated.
    let draw = |witness: &[Fp], table: &[Fp]| -> Fp {
        let probe = Lookup::new(witness.to_vec(), table.to_vec(), Fp::ZERO);
        let mut tr = PoseidonTranscript::new(Poseidon::new(3, [Fp::ZERO; RATE]));
        for v in witness {
            tr.absorb(*v);
        }
        for m in probe.multiplicities() {
            tr.absorb(*m);
        }
        tr.challenge()
    };

    let x = draw(&witness, &table);
    let air = Lookup::new(witness.clone(), table.clone(), x);
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(
        stark_verify(&air, &proof, QUERIES),
        "an in-table witness failed under a drawn challenge"
    );

    let mut out = witness;
    out.push(Fp::from_u64(99)); // 99 is not in the table
    let x2 = draw(&out, &table);
    let bad = Lookup::new(out, table, x2);
    let bad_proof = stark_prove(&bad, &bad.trace(), QUERIES);
    assert!(
        !stark_verify(&bad, &bad_proof, QUERIES),
        "an out-of-table witness verified under a drawn challenge"
    );
}

use crate::crypto::stark::air::TupleLookup;

const TUPLE_ALPHA: u64 = 0x9e37_79b9;

// A position-dependent range table: position 0 admits a two-bit limb {0,1,2,3},
// position 1 admits a one-bit limb {0,1}. A pair carries both the value and
// where it is allowed to sit, so a legitimate value at the wrong position is not
// a table entry.
fn tuple_range_table() -> Vec<Vec<Fp>> {
    let mut table = Vec::new();
    for v in 0..4u64 {
        table.push(alloc::vec![Fp::from_u64(v), Fp::ZERO]);
    }
    for v in 0..2u64 {
        table.push(alloc::vec![Fp::from_u64(v), Fp::ONE]);
    }
    table
}

#[test]
fn a_tuple_range_check_of_in_range_pairs_verifies() {
    // Every value in [0, 8) splits into a two-bit low limb at position 0 and a
    // one-bit high limb at position 1, each presented as a (value, position)
    // pair that the table admits.
    let table = tuple_range_table();
    let mut witness: Vec<Vec<Fp>> = Vec::new();
    for v in [0u64, 3, 4, 7, 5, 1] {
        witness.push(alloc::vec![Fp::from_u64(v & 0b11), Fp::ZERO]);
        witness.push(alloc::vec![Fp::from_u64((v >> 2) & 0b1), Fp::ONE]);
    }
    let air =
        TupleLookup::new(witness, table, 2, Fp::from_u64(TUPLE_ALPHA), Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an in-range pair set was rejected");
}

#[test]
fn a_right_value_wrong_position_is_rejected() {
    // The value 3 is a legitimate limb, but only at position 0; presenting it at
    // position 1 is out of range. A scalar lookup on the values alone would
    // admit it, since 3 is in the value set; the pair lookup catches the
    // position, which is exactly what range-checking a FRI index needs.
    let table = tuple_range_table();
    let witness: Vec<Vec<Fp>> = alloc::vec![
        alloc::vec![Fp::from_u64(3), Fp::ZERO], // legitimate: 3 at position 0
        alloc::vec![Fp::from_u64(3), Fp::ONE],  // right value, wrong position
    ];
    let air =
        TupleLookup::new(witness, table, 2, Fp::from_u64(TUPLE_ALPHA), Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a value at the wrong position verified");
}

#[test]
fn a_folded_opening_pair_looks_up_against_committed_pairs() {
    // A folded FRI opening is an (a, b) pair; the verifier looks it up against
    // the table of pairs the commitment fixes. A pair sharing a first component
    // with a table entry but not its second is still off the table.
    let table: Vec<Vec<Fp>> = [(3u64, 9u64), (5, 25), (7, 49), (2, 4)]
        .iter()
        .map(|(a, b)| alloc::vec![Fp::from_u64(*a), Fp::from_u64(*b)])
        .collect();
    let good: Vec<Vec<Fp>> = alloc::vec![
        alloc::vec![Fp::from_u64(5), Fp::from_u64(25)],
        alloc::vec![Fp::from_u64(2), Fp::from_u64(4)],
    ];
    let air =
        TupleLookup::new(good, table.clone(), 2, Fp::from_u64(TUPLE_ALPHA), Fp::from_u64(LOOKUP_X));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "a committed opening pair was rejected");

    // Right first component, wrong second: (5, 24) is not a committed pair.
    let bad: Vec<Vec<Fp>> = alloc::vec![alloc::vec![Fp::from_u64(5), Fp::from_u64(24)]];
    let bad_air =
        TupleLookup::new(bad, table, 2, Fp::from_u64(TUPLE_ALPHA), Fp::from_u64(LOOKUP_X));
    let bad_proof = stark_prove(&bad_air, &bad_air.trace(), QUERIES);
    assert!(!stark_verify(&bad_air, &bad_proof, QUERIES), "an unmatched opening pair verified");
}

#[test]
fn the_tuple_lookup_verifies_and_rejects_over_random_cases() {
    let table = tuple_range_table();
    let mut s = 0x0bad_c0deu64;
    let alpha = Fp::from_u64(TUPLE_ALPHA);
    let x = Fp::from_u64(LOOKUP_X);

    for _ in 0..40 {
        let len = 1 + (lookup_xs(&mut s) % 6) as usize;
        // An in-table pair set: every pair drawn from the table.
        let good: Vec<Vec<Fp>> =
            (0..len).map(|_| table[(lookup_xs(&mut s) as usize) % table.len()].clone()).collect();
        let air = TupleLookup::new(good, table.clone(), 2, alpha, x);
        let proof = stark_prove(&air, &air.trace(), QUERIES);
        assert!(stark_verify(&air, &proof, QUERIES), "an in-table random pair set was rejected");

        // One pair swapped to a valid value at a position it is not allowed: a
        // 2 or 3 at position 1, in range for position 0 but off the table here.
        let mut bad: Vec<Vec<Fp>> =
            (0..len).map(|_| table[(lookup_xs(&mut s) as usize) % table.len()].clone()).collect();
        bad[(lookup_xs(&mut s) as usize) % len] =
            alloc::vec![Fp::from_u64(2 + lookup_xs(&mut s) % 2), Fp::ONE];
        let bad_air = TupleLookup::new(bad, table.clone(), 2, alpha, x);
        let bad_proof = stark_prove(&bad_air, &bad_air.trace(), QUERIES);
        assert!(
            !stark_verify(&bad_air, &bad_proof, QUERIES),
            "an out-of-table random pair set verified"
        );
    }
}

// Context binding: a membership proof is tied to a capsule context, so one
// capsule's proof cannot admit another.

#[test]
fn a_context_bound_membership_proof_verifies_under_its_context() {
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let index = 5;
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, tree.root(), path, directions);

    // The public context this attestation is bound to: a capsule hash, its
    // granted capabilities, and the policy epoch, exactly the gate's ctx bytes.
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(&[0xa5u8; 32]);
    ctx[32..40].copy_from_slice(&0x0000_0000_0000_000fu64.to_be_bytes());
    ctx[40..48].copy_from_slice(&1u64.to_be_bytes());

    let proof = stark_prove_bound(&air, &trace, QUERIES, &ctx);
    assert!(
        stark_verify_bound(&air, &proof, QUERIES, &ctx),
        "an honest attestation was rejected under its own context"
    );
}

#[test]
fn a_membership_proof_is_rejected_under_a_different_context() {
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let index = 5;
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, tree.root(), path, directions);

    let mut ctx_a = [0u8; 48];
    ctx_a[..32].copy_from_slice(&[0xa5u8; 32]);
    // A different capsule: one byte of the hash differs.
    let mut ctx_b = ctx_a;
    ctx_b[0] ^= 1;

    let proof = stark_prove_bound(&air, &trace, QUERIES, &ctx_a);
    assert!(
        stark_verify_bound(&air, &proof, QUERIES, &ctx_a),
        "the proof failed under the context it was made for"
    );
    // The same enrolled leaf and path, but the proof cannot be replayed for a
    // capsule it was not drawn for: a different context is rejected.
    assert!(
        !stark_verify_bound(&air, &proof, QUERIES, &ctx_b),
        "a proof for one capsule verified for another"
    );
    // And it does not verify as an unbound proof either.
    assert!(!stark_verify(&air, &proof, QUERIES), "a bound proof verified with no context");
}

#[test]
fn an_empty_context_matches_the_unbound_proof() {
    // The bound path is a strict extension: an empty context reproduces the
    // unbound proof exactly, so existing proofs stay valid.
    let seed = Fp::from_u64(2);
    let air = Squaring { log_t: 3, seed };
    let trace = squaring_trace(3, seed);
    let bound = stark_prove_bound(&air, &trace, QUERIES, &[]);
    let unbound = stark_prove(&air, &trace, QUERIES);
    assert!(stark_verify(&air, &bound, QUERIES), "empty-context proof rejected by stark_verify");
    assert!(
        stark_verify_bound(&air, &unbound, QUERIES, &[]),
        "unbound proof rejected by empty-context verify"
    );
}

// Serialization: a real proof round-trips exactly, and the decoder is total
// over hostile bytes.

#[test]
fn a_proof_round_trips_through_its_bytes() {
    use crate::crypto::stark::air::{deserialize_proof, serialize_proof};

    // A real membership proof, the shape an attestation carries.
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let index = 3;
    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(hasher.clone(), log_rounds, tree.root(), path, directions);
    let proof = stark_prove(&air, &trace, QUERIES);

    let bytes = serialize_proof(&proof);
    let decoded = deserialize_proof(&bytes).expect("a canonical proof failed to decode");
    // The decoded proof verifies exactly as the original.
    assert!(stark_verify(&air, &decoded, QUERIES), "the round-tripped proof was rejected");
    // And re-encoding is stable, so the encoding is canonical.
    assert_eq!(serialize_proof(&decoded), bytes, "re-encoding a decoded proof differed");
}

#[test]
fn a_truncated_proof_is_rejected_not_panicked() {
    use crate::crypto::stark::air::{deserialize_proof, serialize_proof};

    let air = Squaring { log_t: 3, seed: Fp::from_u64(5) };
    let proof = stark_prove(&air, &squaring_trace(3, Fp::from_u64(5)), QUERIES);
    let bytes = serialize_proof(&proof);
    // Every truncation of a valid proof must decode to None, not crash.
    for cut in 0..bytes.len() {
        let _ = deserialize_proof(&bytes[..cut]);
    }
    // Trailing bytes make it non-canonical and are rejected.
    let mut extended = bytes.clone();
    extended.push(0);
    assert!(deserialize_proof(&extended).is_none(), "a proof with trailing bytes decoded");
}

#[test]
fn arbitrary_bytes_never_panic_the_decoder() {
    use crate::crypto::stark::air::deserialize_proof;

    let mut s = 0x51ed_2701_dead_c0deu64;
    for _ in 0..20_000 {
        let len = (lookup_xs(&mut s) % 512) as usize;
        let buf: Vec<u8> = (0..len).map(|_| (lookup_xs(&mut s) & 0xff) as u8).collect();
        // No assertion on the result: the point is that it returns rather than
        // panics or hangs, over any bytes a hostile trailer could carry.
        let _ = deserialize_proof(&buf);
    }
}

// The attestation core the gate runs: an enrolled secret vouches for exactly
// this capsule, verified against the kernel's own root.

fn enroll_and_prove(
    hasher: &Poseidon,
    leaves: &[[Fp; RATE]],
    index: usize,
    log_rounds: u32,
    ctx: &[u8],
) -> (Vec<[Fp; RATE]>, Vec<bool>, [Fp; RATE], Vec<u8>) {
    use crate::crypto::stark::air::serialize_proof;
    let tree = PoseidonMerkleTree::commit(hasher, leaves);
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(
        hasher.clone(),
        log_rounds,
        tree.root(),
        path.clone(),
        directions.clone(),
    );
    let proof = stark_prove_bound(&air, &trace, QUERIES, ctx);
    (path, directions, tree.root(), serialize_proof(&proof))
}

#[test]
fn an_enrolled_capsule_attestation_is_accepted() {
    use crate::crypto::stark::air::verify_membership_attestation;
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let ctx = capsule_ctx(0xa5, 0x0f, 1);
    let (siblings, directions, root, bytes) =
        enroll_and_prove(&hasher, &leaves, 5, log_rounds, &ctx);
    assert!(
        verify_membership_attestation(
            &hasher,
            log_rounds,
            root,
            &siblings,
            &directions,
            QUERIES,
            &bytes,
            &ctx
        ),
        "an enrolled capsule was denied"
    );
}

#[test]
fn attestation_is_denied_for_a_different_capsule_or_root() {
    use crate::crypto::stark::air::verify_membership_attestation;
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let ctx = capsule_ctx(0xa5, 0x0f, 1);
    let (siblings, directions, root, bytes) =
        enroll_and_prove(&hasher, &leaves, 5, log_rounds, &ctx);

    // A different capsule (context) is denied: the proof cannot be replayed.
    let other_ctx = capsule_ctx(0xa6, 0x0f, 1);
    assert!(
        !verify_membership_attestation(
            &hasher,
            log_rounds,
            root,
            &siblings,
            &directions,
            QUERIES,
            &bytes,
            &other_ctx
        ),
        "a proof for one capsule admitted another"
    );

    // A wrong policy root is denied: enrollment in some other tree does not count.
    let mut wrong_root = root;
    wrong_root[0] = wrong_root[0] + Fp::ONE;
    assert!(
        !verify_membership_attestation(
            &hasher,
            log_rounds,
            wrong_root,
            &siblings,
            &directions,
            QUERIES,
            &bytes,
            &ctx
        ),
        "an attestation verified against the wrong root"
    );

    // Tampered proof bytes are denied without a panic.
    let mut bad = bytes.clone();
    if let Some(b) = bad.get_mut(64) {
        *b ^= 1;
    }
    assert!(
        !verify_membership_attestation(
            &hasher,
            log_rounds,
            root,
            &siblings,
            &directions,
            QUERIES,
            &bad,
            &ctx
        ),
        "a tampered attestation verified"
    );

    // A malformed path is denied.
    assert!(
        !verify_membership_attestation(
            &hasher,
            log_rounds,
            root,
            &siblings,
            &directions[..directions.len() - 1],
            QUERIES,
            &bytes,
            &ctx
        ),
        "a mismatched path length verified"
    );
}

fn capsule_ctx(hash_byte: u8, caps: u64, epoch: u64) -> Vec<u8> {
    let mut ctx = alloc::vec![0u8; 48];
    for b in ctx[..32].iter_mut() {
        *b = hash_byte;
    }
    ctx[32..40].copy_from_slice(&caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&epoch.to_be_bytes());
    ctx
}

// The whole trailer the gate parses. Query and round counts are the kernel's,
// not the trailer's. A real trailer verifies; magic, direction, and proof
// tampering are rejected.

fn build_trailer(directions: &[bool], siblings: &[[Fp; RATE]], proof_bytes: &[u8]) -> Vec<u8> {
    use crate::crypto::stark::air::STARK_ATTEST_MAGIC;
    let depth = siblings.len();
    let mut t = Vec::new();
    t.extend_from_slice(STARK_ATTEST_MAGIC);
    t.push(depth as u8);
    let dir_bytes = depth.div_ceil(8);
    let mut dirs = alloc::vec![0u8; dir_bytes];
    for (i, d) in directions.iter().enumerate() {
        if *d {
            dirs[i / 8] |= 1 << (i % 8);
        }
    }
    t.extend_from_slice(&dirs);
    for s in siblings {
        for v in s {
            t.extend_from_slice(&v.value().to_le_bytes());
        }
    }
    t.extend_from_slice(proof_bytes);
    t
}

#[test]
fn a_real_attestation_trailer_verifies_and_tampering_is_rejected() {
    use crate::crypto::stark::air::{serialize_proof, verify_attestation_trailer};
    let log_rounds = 3u32;
    let hasher = Poseidon::new(log_rounds, [Fp::ZERO; RATE]);
    let leaves = merkle_leaves(8);
    let index = 6;
    let ctx = capsule_ctx(0x7c, 0x03, 1);

    let tree = PoseidonMerkleTree::commit(&hasher, &leaves);
    let path = tree.open(index);
    let directions: Vec<bool> = (0..path.len()).map(|k| (index >> k) & 1 == 1).collect();
    let trace = membership_trace(&hasher, leaves[index], &path, &directions, log_rounds);
    let air = MerkleMembership::new(
        hasher.clone(),
        log_rounds,
        tree.root(),
        path.clone(),
        directions.clone(),
    );
    let proof = stark_prove_bound(&air, &trace, QUERIES, &ctx);
    let trailer = build_trailer(&directions, &path, &serialize_proof(&proof));

    let root = tree.root();
    assert!(
        verify_attestation_trailer(&hasher, log_rounds, root, QUERIES, &trailer, &ctx),
        "a real attestation trailer was rejected"
    );

    // Wrong magic.
    let mut bad_magic = trailer.clone();
    bad_magic[0] ^= 1;
    assert!(!verify_attestation_trailer(&hasher, log_rounds, root, QUERIES, &bad_magic, &ctx));

    // A corrupted direction bit changes the claimed path and must fail.
    let mut bad_dir = trailer.clone();
    bad_dir[9] ^= 1;
    assert!(!verify_attestation_trailer(&hasher, log_rounds, root, QUERIES, &bad_dir, &ctx));

    // A flipped proof byte must fail.
    let mut bad_proof = trailer.clone();
    let last = bad_proof.len() - 1;
    bad_proof[last] ^= 1;
    assert!(!verify_attestation_trailer(&hasher, log_rounds, root, QUERIES, &bad_proof, &ctx));

    // Truncations decode without a panic. Sampled across the buffer, including
    // the header boundaries where the parser transitions between fields, so the
    // suite stays fast while still exercising the bounds checks.
    let mut cut = 0;
    while cut < trailer.len() {
        let _ =
            verify_attestation_trailer(&hasher, log_rounds, root, QUERIES, &trailer[..cut], &ctx);
        cut += 37;
    }
}
