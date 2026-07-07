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
use crate::crypto::stark::poseidon_transcript::PoseidonTranscript;
use alloc::vec::Vec;

const LOG_ROUNDS: u32 = 3;

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
