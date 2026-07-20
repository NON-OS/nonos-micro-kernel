// NONOS Operating System (AGPL-3.0-or-later)
//! The preprocessed-periodic commit across all cores. The serial
//! `periodic_root` extends 444 columns and hashes a wide leaf per row on one
//! thread; over a 2^24 eval domain that is the wall-clock floor of an emit.
//! This computes the identical root in parallel: the per-column coset LDEs are
//! independent pure maps, the wide leaves hash independently per row, and the
//! tree above the leaves is built by the same shared serial builder. Every
//! parallel stage is an order-preserving map of a pure function, so the root
//! is identical to the serial one by construction; `parallel_tests` proves the
//! equality bit-for-bit on a small AIR. It never feeds a settlement vector
//! until that gate is green.

use crate::crypto::stark::air::{periodic_domain_log, AirExt};
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use crate::crypto::stark::merkle::{hash_leaf_wide_periodic, MerkleTree};
use crate::crypto::stark::poly::lde;
use alloc::vec::Vec;
use rayon::prelude::*;

const SHIFT: u64 = 7;

pub(crate) fn parallel_periodic_root<A: AirExt>(air: &A, extra_blowup_bits: u32) -> [u8; 32] {
    let log_t = air.log_trace_len();
    let log_n = periodic_domain_log(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    // The per-column coset extensions: independent, the NTT-heavy majority of
    // the work, one column per task.
    let cols = air.periodic_columns();
    let extended: Vec<Vec<Fp>> = cols.par_iter().map(|col| lde(col, g, shift, omega, n)).collect();
    let width = extended.len();

    // The leaf count comes from the extended columns, exactly as the serial
    // `commit_rows` takes it from `columns.first().len()`: for one or more
    // columns this is the domain size, and with no columns it is zero, so the
    // two paths agree in every case rather than only the common one.
    let n_leaves = extended.first().map(Vec::len).unwrap_or(0);

    // The wide periodic leaf per row, hashed in parallel; the indexed collect
    // preserves row order, so the leaf layer equals the serial one exactly.
    let leaves: Vec<[u8; 32]> = (0..n_leaves)
        .into_par_iter()
        .map(|i| {
            let row: Vec<Fp> = extended.iter().map(|c| c[i]).collect();
            hash_leaf_wide_periodic(&row)
        })
        .collect();

    // The tree above the leaves, by the shared serial builder. The domain is a
    // power of two, so the pad leaf is never reached.
    let pad = hash_leaf_wide_periodic(&alloc::vec![Fp::ZERO; width]);
    MerkleTree::from_leaf_digests(leaves, pad).root()
}
