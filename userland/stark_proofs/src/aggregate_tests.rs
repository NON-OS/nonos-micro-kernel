// NONOS Operating System (AGPL-3.0-or-later)
//! One proof for the whole capsule set. The kernel verifies it once at boot and
//! then trusts the table it returns, so the table is only as good as what the
//! proof binds. These check that editing any part of it, or presenting it under
//! another root or epoch, leaves nothing attested, and that a malformed blob is
//! refused rather than trusted or panicked on.

use crate::crypto::stark::air::{
    build_aggregate, measure_capsule, verify_aggregate, MeasuredSet, Poseidon, RATE,
};
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

const SLOTS: usize = 8;
const ENROLLED: usize = 5;
const EPOCH: u64 = 1;

fn images() -> Vec<Vec<u8>> {
    (0..SLOTS).map(|k| (0..1024usize).map(|i| (i * 31 + k * 7 + 1) as u8).collect()).collect()
}

fn caps() -> Vec<u64> {
    (0..ENROLLED).map(|i| 0x19u64 | ((i as u64) << 32)).collect()
}

fn hasher() -> Poseidon {
    Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn enrolled() -> (Poseidon, MeasuredSet, Vec<u8>) {
    let h = hasher();
    let imgs = images();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    let set = MeasuredSet::commit(&h, &refs);
    let blob = build_aggregate(
        &h,
        LOG_ROUNDS,
        &set,
        &caps(),
        EPOCH,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    )
    .expect("the set should enroll");
    (h, set, blob)
}

fn check(h: &Poseidon, root: [Fp; RATE], blob: &[u8]) -> bool {
    verify_aggregate(h, LOG_ROUNDS, root, blob, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP_BITS).is_some()
}

#[test]
fn the_whole_set_verifies_under_one_proof() {
    let (h, set, blob) = enrolled();
    let table = verify_aggregate(
        &h,
        LOG_ROUNDS,
        set.root(),
        &blob,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    )
    .expect("the aggregate should verify");
    assert_eq!(table.len(), ENROLLED);
    for (i, e) in table.iter().enumerate() {
        assert_eq!(e.caps, caps()[i], "entry {i} carries the wrong capabilities");
        let leaf = measure_capsule(&h, &images()[i]);
        let mut expect = [0u8; 32];
        for (j, lane) in leaf.iter().enumerate() {
            expect[j * 8..j * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
        }
        assert_eq!(e.measurement, expect, "entry {i} does not measure to its image");
    }
}

#[test]
fn raising_a_capability_word_breaks_the_proof() {
    let (h, set, mut blob) = enrolled();
    // The capabilities of entry 0 sit right after its measurement.
    let at = 19 + 32;
    blob[at + 7] ^= 0x20;
    assert!(!check(&h, set.root(), &blob), "an edited capability word still verified");
}

#[test]
fn swapping_a_measurement_breaks_the_proof() {
    let (h, set, mut blob) = enrolled();
    blob[19] ^= 0x01;
    assert!(!check(&h, set.root(), &blob), "an edited measurement still verified");
}

#[test]
fn another_root_attests_nothing() {
    let (h, set, blob) = enrolled();
    let mut wrong = set.root();
    wrong[0] = wrong[0] + Fp::from_u64(1);
    assert!(!check(&h, wrong, &blob), "the set verified under a root it never opened");
}

#[test]
fn replaying_under_another_epoch_fails() {
    let (h, set, mut blob) = enrolled();
    blob[15] ^= 0x01;
    assert!(!check(&h, set.root(), &blob), "the aggregate replayed under another epoch");
}

#[test]
fn a_truncated_or_empty_blob_is_refused() {
    let (h, set, blob) = enrolled();
    for cut in [0usize, 8, 19, 19 + 32, blob.len() / 2, blob.len() - 1] {
        assert!(!check(&h, set.root(), &blob[..cut]), "a blob cut at {cut} verified");
    }
}

#[test]
fn a_foreign_magic_is_refused() {
    let (h, set, mut blob) = enrolled();
    blob[0] = b'X';
    assert!(!check(&h, set.root(), &blob), "a blob with the wrong magic verified");
}

#[test]
fn a_swapped_sibling_path_breaks_the_proof() {
    let (h, set, mut blob) = enrolled();
    let paths_at = 19 + ENROLLED * 40;
    blob[paths_at] ^= 0x01;
    assert!(!check(&h, set.root(), &blob), "an edited sibling path still verified");
}
