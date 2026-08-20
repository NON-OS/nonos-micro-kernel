// NONOS Operating System (AGPL-3.0-or-later)
//! Enrollment measures every capsule image with a Poseidon sponge over its whole
//! length. Opening one capsule needs the same committed tree as opening any
//! other, so the set is measured once and shared. These pin that the shared path
//! is not a different proof: same root, same trailer bytes, still gate-verifiable.

use crate::crypto::stark::air::{
    build_attestation_trailer, build_attestation_trailer_from_set, enroll_policy_root,
    verify_membership_trailer, MeasuredSet, Poseidon, RATE,
};
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

const N: usize = 8;
/// Tree depth for N leaves.
const DEPTH: usize = 3;

fn images() -> Vec<Vec<u8>> {
    (0..N).map(|k| (0..2048usize).map(|i| (i * 31 + k * 7 + 1) as u8).collect()).collect()
}

fn context(caps: u64) -> Vec<u8> {
    let mut ctx = alloc::vec![0u8; 48];
    for (i, b) in ctx.iter_mut().enumerate().take(32) {
        *b = i as u8;
    }
    ctx[32..40].copy_from_slice(&caps.to_be_bytes());
    ctx
}

/// The root as the gate holds it: each lane little-endian.
fn root_bytes(root: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in root.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

fn hasher() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}

#[test]
fn committing_once_gives_the_same_root() {
    let imgs = images();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    let h = hasher();
    assert_eq!(MeasuredSet::commit(&h, &refs).root(), enroll_policy_root(&h, &refs));
}

#[test]
fn the_shared_set_yields_byte_identical_trailers() {
    let imgs = images();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    let h = hasher();
    let set = MeasuredSet::commit(&h, &refs);
    for i in 0..N {
        let ctx = context(i as u64);
        let slow = build_attestation_trailer(
            &h,
            LOG_ROUNDS,
            &refs,
            i,
            &ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        );
        let fast = build_attestation_trailer_from_set(
            &h,
            LOG_ROUNDS,
            &set,
            i,
            &ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        );
        assert_eq!(slow, fast, "trailer {i} differs on the shared path");
    }
}

#[test]
fn every_shared_trailer_still_verifies_against_the_root() {
    let imgs = images();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    let h = hasher();
    let set = MeasuredSet::commit(&h, &refs);
    let root = root_bytes(set.root());
    for i in 0..N {
        let ctx = context(i as u64);
        let trailer = build_attestation_trailer_from_set(
            &h,
            LOG_ROUNDS,
            &set,
            i,
            &ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        );
        assert!(
            verify_membership_trailer(
                &h,
                LOG_ROUNDS,
                root,
                DEPTH,
                &trailer,
                &ctx,
                N_QUERIES,
                GRIND_BITS,
                EXTRA_BLOWUP_BITS
            ),
            "trailer {i} did not verify"
        );
    }
}

#[test]
fn a_trailer_does_not_verify_under_another_capsules_context() {
    let imgs = images();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    let h = hasher();
    let set = MeasuredSet::commit(&h, &refs);
    let root = root_bytes(set.root());
    let trailer = build_attestation_trailer_from_set(
        &h,
        LOG_ROUNDS,
        &set,
        0,
        &context(0),
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    assert!(
        !verify_membership_trailer(
            &h,
            LOG_ROUNDS,
            root,
            DEPTH,
            &trailer,
            &context(1),
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS
        ),
        "a trailer verified under a context it was not bound to"
    );
}
