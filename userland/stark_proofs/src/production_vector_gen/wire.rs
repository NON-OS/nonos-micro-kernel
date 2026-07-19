// NONOS Operating System (AGPL-3.0-or-later)
//! The wire layout of the preprocessed-periodic proof: the frozen base proof
//! bytes (spec/stark-serialization.md), then the periodic sidecar — the
//! claims at z, and per consistency query the wide periodic row with its
//! path against the baked root.

use crate::crypto::stark::air::StarkProofExtPre;
use alloc::vec::Vec;

pub(super) fn serialize_pre(pre: &StarkProofExtPre) -> Vec<u8> {
    let mut b = crate::stark_selftest_gen::serialize(&pre.proof);
    // sidecar: n_periodic, the claims at z, then one opening per query in
    // query order: n_periodic row values, then the path.
    b.extend_from_slice(&(pre.periodic_z.len() as u32).to_le_bytes());
    for v in &pre.periodic_z {
        b.extend_from_slice(&v.c0.value().to_le_bytes());
        b.extend_from_slice(&v.c1.value().to_le_bytes());
    }
    for op in &pre.openings {
        for v in &op.row {
            b.extend_from_slice(&v.value().to_le_bytes());
        }
        b.extend_from_slice(&(op.path.len() as u32).to_le_bytes());
        for d in &op.path {
            b.extend_from_slice(d);
        }
    }
    b
}
