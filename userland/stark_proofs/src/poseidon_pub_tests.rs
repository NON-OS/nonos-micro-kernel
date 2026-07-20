// NONOS Operating System (AGPL-3.0-or-later)
//! The publics-bound Poseidon pair: a proof drawn with
//! `stark_prove_poseidon_ext_pub` verifies only through the matching `_pub`
//! verifier replaying the same publics, and rejects under different publics
//! or the unbound verifier.

use crate::crypto::stark::air::{
    stark_prove_poseidon_ext_pub, stark_verify_poseidon_ext, stark_verify_poseidon_ext_pub,
    Poseidon, Squaring, RATE,
};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

fn hasher() -> Poseidon {
    Poseidon::new(2, [Fp::ZERO; RATE])
}

fn squaring_trace(log_t: u32, seed: Fp) -> Vec<Fp> {
    let mut trace = Vec::with_capacity(1usize << log_t);
    let mut cur = seed;
    for _ in 0..1usize << log_t {
        trace.push(cur);
        cur = cur * cur;
    }
    trace
}

#[test]
fn a_publics_bound_proof_verifies_under_the_same_publics() {
    let seed = Fp::from_u64(5);
    let air = Squaring { log_t: 4, seed };
    let trace = squaring_trace(4, seed);
    let h = hasher();
    let publics: Vec<Fp> = (0..4).map(|i| Fp::from_u64(0xB000 + i)).collect();
    let proof = stark_prove_poseidon_ext_pub(&air, &trace, 32, 8, 0, &h, &publics);
    assert!(
        stark_verify_poseidon_ext_pub(&air, &proof, 32, 8, 0, &h, &publics),
        "an honest publics-bound proof was rejected"
    );
}

#[test]
fn a_publics_bound_proof_rejects_different_publics() {
    let seed = Fp::from_u64(5);
    let air = Squaring { log_t: 4, seed };
    let trace = squaring_trace(4, seed);
    let h = hasher();
    let publics: Vec<Fp> = (0..4).map(|i| Fp::from_u64(0xB000 + i)).collect();
    let proof = stark_prove_poseidon_ext_pub(&air, &trace, 32, 8, 0, &h, &publics);
    let mut wrong = publics.clone();
    wrong[0] = wrong[0] + Fp::ONE;
    assert!(
        !stark_verify_poseidon_ext_pub(&air, &proof, 32, 8, 0, &h, &wrong),
        "a publics-bound proof verified under different publics"
    );
    assert!(
        !stark_verify_poseidon_ext(&air, &proof, 32, 8, 0, &h),
        "a publics-bound proof verified without its publics"
    );
}
