// NONOS Operating System (AGPL-3.0-or-later)
//! The preprocessed-periodic pair against the wired join-split shape: the
//! honest proof verifies against the baked periodic root, and each targeted
//! tamper (a claim off the true evaluation, a mutated opening, a wrong root)
//! is rejected.

use crate::crypto::stark::air::{
    stark_prove_ext_preprocessed, stark_verify_ext_preprocessed, Accumulator, Air, AirExt,
    RangeCheck, StarkProofExtPre, WiredExt,
};
use crate::crypto::stark::field::{Fp, Fp2};
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::merkle::MerkleTree;
use crate::crypto::stark::poly::lde;
use alloc::boxed::Box;
use alloc::vec::Vec;

fn neg(x: u64) -> Fp {
    Fp::ZERO - Fp::from_u64(x)
}

fn join_split_air() -> WiredExt {
    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(Accumulator { log_t: 3 }) as Box<dyn AirExt>,
        Box::new(RangeCheck { log_t: 4 }),
    ];
    let mut sigma: Vec<usize> = (0..32).collect();
    sigma.swap(1, 8);
    WiredExt::new(regions, alloc::vec![0], sigma, Fp::from_u64(5), Fp::from_u64(7))
}

fn witness(air: &WiredExt) -> Vec<Fp> {
    let addends =
        [Fp::from_u64(7), Fp::from_u64(3), neg(8), neg(1), neg(1), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    let mut cons = Vec::with_capacity(addends.len() * 2);
    let mut acc = Fp::ZERO;
    for &a in &addends {
        cons.push(acc);
        cons.push(a);
        acc = acc + a;
    }
    let mut rng = Vec::with_capacity(32);
    let mut v = 7u64;
    for i in 0..16usize {
        let bit = if i < 15 { v & 1 } else { 0 };
        rng.push(Fp::from_u64(v));
        rng.push(Fp::from_u64(bit));
        if i < 15 {
            v >>= 1;
        }
    }
    air.trace(&[cons, rng])
}

/// The baked commitment a deployment verifier holds: the periodic columns
/// extended to the eval domain and committed under the periodic leaf domain.
fn baked_root(air: &WiredExt, extra_blowup_bits: u32) -> [u8; 32] {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let bound = (air.constraint_degree().max(1) * t).next_power_of_two();
    let n = (2 * bound) << extra_blowup_bits;
    let g = root_of_unity(log_t);
    let omega = root_of_unity(n.trailing_zeros());
    let shift = Fp::from_u64(7);
    let extended: Vec<Vec<Fp>> =
        air.periodic_columns().iter().map(|c| lde(c, g, shift, omega, n)).collect();
    MerkleTree::commit_wide_periodic(&extended).root()
}

fn setup() -> (WiredExt, StarkProofExtPre, [u8; 32]) {
    let air = join_split_air();
    let w = witness(&air);
    let proof = stark_prove_ext_preprocessed(&air, &w, 32, 8, 0);
    let root = baked_root(&air, 0);
    (air, proof, root)
}

#[test]
fn the_preprocessed_proof_verifies_against_the_baked_root() {
    let (air, proof, root) = setup();
    assert!(
        stark_verify_ext_preprocessed(&air, &proof, 32, 8, 0, &root),
        "an honest preprocessed proof was rejected"
    );
}

#[test]
fn a_tampered_periodic_claim_is_rejected() {
    let (air, mut proof, root) = setup();
    proof.periodic_z[0] = proof.periodic_z[0] + Fp2::ONE;
    assert!(
        !stark_verify_ext_preprocessed(&air, &proof, 32, 8, 0, &root),
        "a periodic claim off the true evaluation verified"
    );
}

#[test]
fn a_tampered_periodic_opening_is_rejected() {
    let (air, mut proof, root) = setup();
    proof.openings[0].row[0] = proof.openings[0].row[0] + Fp::ONE;
    assert!(
        !stark_verify_ext_preprocessed(&air, &proof, 32, 8, 0, &root),
        "a mutated periodic opening verified"
    );
}

#[test]
fn a_wrong_periodic_root_is_rejected() {
    let (air, proof, mut root) = setup();
    root[0] ^= 1;
    assert!(
        !stark_verify_ext_preprocessed(&air, &proof, 32, 8, 0, &root),
        "a proof verified against the wrong periodic root"
    );
}
