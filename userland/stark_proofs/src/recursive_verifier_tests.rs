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
