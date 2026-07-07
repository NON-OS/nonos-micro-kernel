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
