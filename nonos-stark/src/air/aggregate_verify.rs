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

//! The kernel side of the set attestation. Run once at boot against the policy
//! root the kernel holds: on success the returned table is what the proof
//! attests, and a spawn is then a lookup in it rather than another proof.
//!
//! Reads only the blob and the trusted root, and returns `None` on anything it
//! cannot account for rather than panicking.

use super::super::field::Fp;
use super::aggregate::{
    digest_from_bytes, parse_entries, parse_header, table_context, Entry, NODE,
};
use super::deserialize_ext::deserialize_proof_ext;
use super::multi_membership::{MultiMembership, Opening};
use super::poseidon::{Poseidon, RATE};
use super::verify_ext::stark_verify_ext_blown_bound;
use alloc::vec::Vec;

/// Verify an aggregate against the trusted `root`, returning the table it
/// attests. `None` means the blob is malformed or the proof does not hold, which
/// the caller must treat the same way: nothing in the set is attested.
#[must_use = "an attestation result must gate the set"]
pub fn verify_aggregate(
    hasher: &Poseidon,
    log_rounds: u32,
    root: [Fp; RATE],
    blob: &[u8],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> Option<Vec<Entry>> {
    let l = parse_header(blob)?;
    let entries = parse_entries(blob, &l)?;

    let mut openings = Vec::with_capacity(l.count);
    for i in 0..l.count {
        let leaf = digest_from_bytes(&blob[l.entries_at + i * 40..l.entries_at + i * 40 + 32])?;
        let mut siblings = Vec::with_capacity(l.depth);
        for k in 0..l.depth {
            let at = l.paths_at + (i * l.depth + k) * NODE;
            siblings.push(digest_from_bytes(&blob[at..at + NODE])?);
        }
        openings.push(Opening {
            leaf,
            root,
            siblings,
            // Derived from the slot, never read from the blob.
            directions: (0..l.depth).map(|k| (i >> k) & 1 == 1).collect(),
        });
    }

    let proof = deserialize_proof_ext(&blob[l.proof_at..])?;
    let context = table_context(l.epoch, &entries);
    let air = MultiMembership::new_shared_root(hasher.clone(), log_rounds, root, openings);
    if !stark_verify_ext_blown_bound(
        &air,
        &proof,
        n_queries,
        grind_bits,
        extra_blowup_bits,
        &context,
    ) {
        return None;
    }
    Some(entries)
}

/// The capabilities slot `i` is enrolled with, if its measurement is the one
/// given. Both must match: a capsule whose bytes changed is not the enrolled
/// one, and a capsule asking for more than it was enrolled with is not either.
pub fn enrolled_caps(table: &[Entry], index: usize, measurement: &[u8; 32]) -> Option<u64> {
    let e = table.get(index)?;
    if &e.measurement != measurement {
        return None;
    }
    Some(e.caps)
}
