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
    stark_prove, stark_verify, FiatShamir, Fibonacci, FriFold, MerkleMembership, MultiMembership,
    Opening, Permutation, Permutation2, Poseidon, PowerChain, Squaring, RATE, WIDTH,
};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::poseidon_merkle::PoseidonMerkleTree;

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
fn multi_trace(hasher: &Poseidon, openings: &[Opening], log_rounds: u32) -> Vec<Fp> {
    let l = 1usize << log_rounds;
    let depth = openings[0].siblings.len();
    let slots = (depth + 1).next_power_of_two();
    let span = slots * l;
    let count = openings.len();
    let batch = count.next_power_of_two().max(1);
    let n = batch * span;

    let start = |o: &Opening| inject_full(o.leaf, o.siblings[0], o.directions[0]);
    let mut rows: Vec<[Fp; WIDTH]> = Vec::with_capacity(n);
    let mut state = start(&openings[0]);
    for r in 0..n {
        rows.push(state);
        let pr = hasher.round_with_rc(&state, &hasher.round_constant(r % l));
        let opening = r / span;
        let within = r % span;
        let at_row_bnd = within % l == l - 1;
        let is_op_bnd = within == span - 1 && opening + 1 < count;
        let is_slot_bnd = at_row_bnd && within < depth * l && !is_op_bnd;
        if is_op_bnd {
            state = start(&openings[opening + 1]);
        } else if is_slot_bnd {
            let m = (within + 1) / l;
            let mut digest = [Fp::ZERO; RATE];
            digest.copy_from_slice(&pr[..RATE]);
            if opening < count && m < depth {
                state = inject_full(
                    digest,
                    openings[opening].siblings[m],
                    openings[opening].directions[m],
                );
            } else {
                state = inject_full(digest, [Fp::ZERO; RATE], false);
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

fn inject_full(node: [Fp; RATE], sibling: [Fp; RATE], right: bool) -> [Fp; WIDTH] {
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
    let trace = multi_trace(&hasher, &openings, log_rounds);
    let air = MultiMembership::new(hasher.clone(), log_rounds, openings);
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
    let openings_true = alloc::vec![opening_at(&tree, &leaves, 2), opening_at(&tree, &leaves, 6)];
    let trace = multi_trace(&hasher, &openings_true, log_rounds);
    o0.root[0] = o0.root[0] + Fp::ONE; // corrupt the first opening's claimed root
    let air = MultiMembership::new(hasher.clone(), log_rounds, alloc::vec![o0, o1]);
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
        let trace = multi_trace(&hasher, &openings, log_rounds);
        let air = MultiMembership::new(hasher.clone(), log_rounds, openings);
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
