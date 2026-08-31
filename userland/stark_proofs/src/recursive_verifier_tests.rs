// NONOS Operating System (AGPL-3.0-or-later)
use crate::crypto::stark::air::{stark_prove, stark_verify, FinalLayerConstant};
use crate::crypto::stark::field::Fp;

extern crate alloc;

const QUERIES: usize = 32;

// The recursive verifier's low-degree conclusion: a FRI final layer is a
// single repeated constant equal to the committed value. A high-degree inner
// proof cannot fold to a constant layer, so this is the gate that turns the
// FRI checks into a real low-degree claim. These prove the check in-circuit
// accepts an honest constant layer and rejects a layer that is not constant or
// does not match its commitment.

#[test]
fn an_honest_constant_final_layer_verifies() {
    let air = FinalLayerConstant::new(4, Fp::from_u64(0xC0FFEE));
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest constant layer was rejected");
}

#[test]
fn a_non_constant_final_layer_is_rejected() {
    let air = FinalLayerConstant::new(4, Fp::from_u64(0xC0FFEE));
    let mut trace = air.trace();
    // A single differing value breaks constancy at one interior row.
    trace[5] = trace[5] + Fp::ONE;
    let proof = stark_prove(&air, &trace, QUERIES);
    assert!(!stark_verify(&air, &proof, QUERIES), "a non-constant layer verified");
}

#[test]
fn a_final_layer_not_matching_its_commitment_is_rejected() {
    // The layer is constant, but the committed value the boundary pins differs,
    // so the proof for the claimed value must fail.
    let honest = FinalLayerConstant::new(4, Fp::from_u64(7));
    let trace = honest.trace();
    let claimed = FinalLayerConstant::new(4, Fp::from_u64(8));
    let proof = stark_prove(&claimed, &trace, QUERIES);
    assert!(!stark_verify(&claimed, &proof, QUERIES), "a mismatched commitment verified");
}

#[test]
fn the_conclusion_holds_across_layer_sizes() {
    for log_len in [1u32, 2, 3, 5] {
        let air = FinalLayerConstant::new(log_len, Fp::from_u64(100 + log_len as u64));
        let proof = stark_prove(&air, &air.trace(), QUERIES);
        assert!(stark_verify(&air, &proof, QUERIES), "honest layer at log_len {log_len} rejected");
    }
}

// Step 3: the transcript-driven challenge derivation. The recursive verifier
// re-derives a FRI proof's fold challenges and query challenges in-circuit
// from the committed roots and final layer, so they are recomputed rather than
// trusted. These prove the in-circuit derivation matches the reference
// transcript, and that a tampered root or a wrongly pinned challenge is
// rejected.

use crate::crypto::stark::air::{FriTranscript, Poseidon, RATE};
use crate::crypto::stark::attest_params::LOG_ROUNDS;
use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
use alloc::vec::Vec;

fn sample_roots(n_folds: usize) -> Vec<[Fp; RATE]> {
    (0..n_folds)
        .map(|i| core::array::from_fn(|j| Fp::from_u64((17 * i + 3 * j + 1) as u64)))
        .collect()
}

fn reference_transcript(
    roots: &[[Fp; RATE]],
    final_layer: &[Fp],
    n_queries: usize,
) -> (Vec<Fp>, Vec<Fp>) {
    // The exact schedule the recursion-ready FRI verifier runs.
    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut t = PoseidonTranscript::new(hasher);
    let mut betas = Vec::new();
    for root in roots {
        t.absorb_digest(root);
        betas.push(t.challenge());
    }
    for v in final_layer {
        t.absorb(*v);
    }
    let mut qch = Vec::new();
    for _ in 0..n_queries {
        qch.push(t.challenge());
    }
    (betas, qch)
}

#[test]
fn the_in_circuit_derivation_matches_the_reference_transcript() {
    let roots = sample_roots(3);
    let final_layer = [Fp::from_u64(90), Fp::from_u64(91)];
    let n_queries = 6;
    let (ref_betas, ref_qch) = reference_transcript(&roots, &final_layer, n_queries);

    let air = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        roots,
        final_layer.to_vec(),
        n_queries,
    );
    // The AIR's own derivation equals the reference transcript's.
    assert_eq!(air.betas(), ref_betas.as_slice(), "derived betas differ from the transcript");
    assert_eq!(
        air.query_challenges(),
        ref_qch.as_slice(),
        "derived query challenges differ from the transcript"
    );
}

#[test]
fn the_beta_rows_point_at_the_squeezed_challenges() {
    // The wiring engine binds a fold's challenge to the transcript by reading
    // the squeezed challenge out of the transcript trace. This checks the map
    // the accessor hands out: each beta row holds that layer's beta in column
    // zero, and each query-challenge row holds its challenge, so the engine can
    // wire (row, 0) directly with no repositioning.
    use crate::crypto::stark::air::WIDTH;

    let roots = sample_roots(3);
    let final_layer = [Fp::from_u64(90), Fp::from_u64(91)];
    let n_queries = 6;
    let air = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        roots,
        final_layer.to_vec(),
        n_queries,
    );
    let trace = air.trace();

    let beta_rows = air.beta_rows();
    assert_eq!(beta_rows.len(), air.betas().len(), "one beta row per fold challenge");
    for (i, row) in beta_rows.iter().enumerate() {
        assert_eq!(trace[row * WIDTH], air.betas()[i], "beta row does not hold its challenge");
    }

    let query_rows = air.query_challenge_rows();
    assert_eq!(query_rows.len(), air.query_challenges().len(), "one row per query challenge");
    for (i, row) in query_rows.iter().enumerate() {
        assert_eq!(
            trace[row * WIDTH],
            air.query_challenges()[i],
            "query row does not hold its challenge"
        );
    }
}

#[test]
fn an_honest_transcript_derivation_verifies() {
    let roots = sample_roots(3);
    let final_layer = [Fp::from_u64(5), Fp::from_u64(6)];
    let air = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        roots,
        final_layer.to_vec(),
        6,
    );
    let proof = stark_prove(&air, &air.trace(), QUERIES);
    assert!(stark_verify(&air, &proof, QUERIES), "an honest transcript derivation was rejected");
}

#[test]
fn a_trace_from_tampered_roots_is_rejected() {
    // Pin the challenges the honest roots derive, but feed a trace built from a
    // tampered root: the squeeze at the affected layer no longer matches.
    let roots = sample_roots(3);
    let final_layer = [Fp::from_u64(5), Fp::from_u64(6)];
    let honest = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        roots.clone(),
        final_layer.to_vec(),
        6,
    );
    let mut tampered = roots;
    tampered[1][0] = tampered[1][0] + Fp::ONE;
    let bad_trace = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        tampered,
        final_layer.to_vec(),
        6,
    )
    .trace();
    let proof = stark_prove(&honest, &bad_trace, QUERIES);
    assert!(!stark_verify(&honest, &proof, QUERIES), "a tampered-root trace verified");
}

#[test]
fn a_wrongly_pinned_challenge_is_rejected() {
    let roots = sample_roots(3);
    let final_layer = [Fp::from_u64(5), Fp::from_u64(6)];
    let air = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        roots.clone(),
        final_layer.to_vec(),
        6,
    );
    let trace = air.trace();
    // Rebuild with the same public inputs but corrupt one pinned beta by
    // reaching through a second instance whose final layer differs, so its
    // squeeze points disagree with this honest trace.
    let other = FriTranscript::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        roots,
        [Fp::from_u64(5), Fp::from_u64(7)].to_vec(),
        6,
    );
    let proof = stark_prove(&other, &trace, QUERIES);
    assert!(!stark_verify(&other, &proof, QUERIES), "a mismatched pin verified");
}

// Step 4: the whole recursive verifier over a real inner FRI proof. The fold
// challenges are re-derived by the transcript AIR, the openings at every query
// and layer are proven committed by the membership AIR, and the folds are the
// public linear check on the re-derived challenges, ending on the constant
// final layer. Every expensive part is a STARK, so this verification is itself
// STARK-provable: recursion.

use crate::crypto::stark::air::{
    layer_openings, recursive_verify, MultiMembership,
};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::fri_poseidon::{fri_prove, FriProof};

// The standard opening-trace builder, mirrored here as test scaffolding to
// produce the membership proofs the recursive verifier consumes.
fn inject_full(node: [Fp; RATE], sibling: [Fp; RATE], right: bool) -> [Fp; 8] {
    let mut state = [Fp::ZERO; 8];
    if !right {
        state[..RATE].copy_from_slice(&node);
        state[RATE..].copy_from_slice(&sibling);
    } else {
        state[..RATE].copy_from_slice(&sibling);
        state[RATE..].copy_from_slice(&node);
    }
    state
}

fn multi_trace(
    hasher: &Poseidon,
    openings: &[crate::crypto::stark::air::Opening],
    log_rounds: u32,
) -> Vec<Fp> {
    let l = 1usize << log_rounds;
    let depth = openings[0].siblings.len();
    let slots = (depth + 1).next_power_of_two();
    let span = slots * l;
    let count = openings.len();
    let batch = count.next_power_of_two().max(1);
    let n = batch * span;
    let start = |o: &crate::crypto::stark::air::Opening| {
        inject_full(o.leaf, o.siblings[0], o.directions[0])
    };
    let mut rows: Vec<[Fp; 8]> = Vec::with_capacity(n);
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
                state = inject_full(digest, openings[opening].siblings[m], openings[opening].directions[m]);
            } else {
                state = inject_full(digest, [Fp::ZERO; RATE], false);
            }
        } else {
            state = pr;
        }
    }
    let mut trace = Vec::with_capacity(n * 8);
    for row in &rows {
        trace.extend_from_slice(row);
    }
    trace
}

const RV_LOG_ROUNDS: u32 = 3;
const RV_STARK_QUERIES: usize = 16;

// A small real inner FRI proof: a low-degree codeword committed with Poseidon.
fn inner_proof() -> (Poseidon, FriProof, u32, u32, usize, Fp) {
    let hasher = Poseidon::new(RV_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let (log_n, log_blowup, n_queries) = (4u32, 1u32, 2usize); // domain 16, fold to 2
    let shift = Fp::from_u64(7);
    // A low-degree codeword: a degree-3 polynomial on the coset.
    let coeffs = [Fp::from_u64(2), Fp::from_u64(5), Fp::from_u64(1), Fp::from_u64(4)];
    let omega = root_of_unity(log_n);
    let n = 1usize << log_n;
    let mut codeword = Vec::with_capacity(n);
    let mut x = shift;
    for _ in 0..n {
        let mut acc = Fp::ZERO;
        for c in coeffs.iter().rev() {
            acc = acc * x + *c;
        }
        codeword.push(acc);
        x = x * omega;
    }
    let proof = fri_prove(&codeword, shift, log_blowup, n_queries, &hasher);
    (hasher, proof, log_n, log_blowup, n_queries, shift)
}

// Build the transcript proof and one membership proof per (query, layer).
fn component_proofs(
    hasher: &Poseidon,
    proof: &FriProof,
    log_n: u32,
    log_blowup: u32,
    n_queries: usize,
) -> (crate::crypto::stark::air::StarkProof, Vec<crate::crypto::stark::air::StarkProof>) {
    let n_folds = (log_n - log_blowup) as usize;
    let n = 1usize << log_n;

    let transcript = FriTranscript::new(
        hasher.clone(),
        RV_LOG_ROUNDS,
        proof.roots.clone(),
        proof.final_layer.to_vec(),
        n_queries,
    );
    let t_proof = stark_prove(&transcript, &transcript.trace(), RV_STARK_QUERIES);

    let mut mproofs = Vec::with_capacity(n_queries * n_folds);
    for q in 0..n_queries {
        let index = (transcript.query_challenges()[q].value() as usize) & (n - 1);
        for m in 0..n_folds {
            let openings = layer_openings(proof, q, m, index, log_n);
            let trace = multi_trace(hasher, &openings, RV_LOG_ROUNDS);
            let air = MultiMembership::new(hasher.clone(), RV_LOG_ROUNDS, openings);
            mproofs.push(stark_prove(&air, &trace, RV_STARK_QUERIES));
        }
    }
    (t_proof, mproofs)
}

#[test]
fn a_real_inner_proof_verifies_recursively() {
    let (hasher, proof, log_n, log_blowup, n_queries, shift) = inner_proof();
    let (t_proof, mproofs) = component_proofs(&hasher, &proof, log_n, log_blowup, n_queries);
    assert!(
        recursive_verify(
            &proof, &hasher, RV_LOG_ROUNDS, shift, log_n, log_blowup, n_queries,
            RV_STARK_QUERIES, &t_proof, &mproofs,
        ),
        "an honest inner proof failed recursive verification"
    );
}

#[test]
fn a_tampered_opening_fails_recursive_verification() {
    let (hasher, mut proof, log_n, log_blowup, n_queries, shift) = inner_proof();
    let (t_proof, mproofs) = component_proofs(&hasher, &proof, log_n, log_blowup, n_queries);
    // Corrupt an opened value after the component proofs were built.
    proof.queries[0].layers[0].a = proof.queries[0].layers[0].a + Fp::ONE;
    assert!(
        !recursive_verify(
            &proof, &hasher, RV_LOG_ROUNDS, shift, log_n, log_blowup, n_queries,
            RV_STARK_QUERIES, &t_proof, &mproofs,
        ),
        "a tampered opening passed recursive verification"
    );
}

#[test]
fn a_tampered_final_layer_fails_recursive_verification() {
    let (hasher, mut proof, log_n, log_blowup, n_queries, shift) = inner_proof();
    let (t_proof, mproofs) = component_proofs(&hasher, &proof, log_n, log_blowup, n_queries);
    proof.final_layer[0] = proof.final_layer[0] + Fp::ONE;
    assert!(
        !recursive_verify(
            &proof, &hasher, RV_LOG_ROUNDS, shift, log_n, log_blowup, n_queries,
            RV_STARK_QUERIES, &t_proof, &mproofs,
        ),
        "a tampered final layer passed recursive verification"
    );
}
