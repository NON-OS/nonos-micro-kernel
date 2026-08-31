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

//! The prover side of the set attestation: open every enrolled capsule against
//! the one policy root, prove the whole batch at once, and pack the table and
//! proof in the layout the kernel parses.

use super::aggregate::{digest_to_bytes, table_context, Entry, AGGREGATE_MAGIC};
use super::attest_build::MeasuredSet;
use super::multi_membership::{MultiMembership, Opening};
use super::poseidon::Poseidon;
use super::prove_ext::stark_prove_ext_blown_bound;
use super::serialize_ext::serialize_proof_ext;
use alloc::vec::Vec;

/// Build the aggregate attestation for the first `caps.len()` slots of `set`.
///
/// `caps[i]` is the capability word capsule `i` is enrolled with. The returned
/// blob carries the table and one proof; the policy root is not in it, since the
/// kernel holds its own and enrollment publishes the same one.
#[allow(clippy::too_many_arguments)]
pub fn build_aggregate(
    hasher: &Poseidon,
    log_rounds: u32,
    set: &MeasuredSet,
    caps: &[u64],
    epoch: u64,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> Option<Vec<u8>> {
    let count = caps.len();
    if count == 0 || count > u16::MAX as usize {
        return None;
    }

    let mut openings = Vec::with_capacity(count);
    let mut entries = Vec::with_capacity(count);
    let mut depth = 0usize;
    for (i, &c) in caps.iter().enumerate() {
        let leaf = set.leaf(i)?;
        let siblings = set.path(i)?;
        depth = siblings.len();
        entries.push(Entry { measurement: digest_to_bytes(leaf), caps: c });
        openings.push(Opening {
            leaf,
            root: set.root(),
            siblings,
            directions: (0..depth).map(|k| (i >> k) & 1 == 1).collect(),
        });
    }
    if depth == 0 || depth > u8::MAX as usize {
        return None;
    }

    let context = table_context(epoch, &entries);
    let air = MultiMembership::new_shared_root(hasher.clone(), log_rounds, set.root(), openings);
    let trace = air.trace();
    let proof = stark_prove_ext_blown_bound(
        &air,
        &trace,
        n_queries,
        grind_bits,
        extra_blowup_bits,
        &context,
    );

    let mut out = Vec::new();
    out.extend_from_slice(AGGREGATE_MAGIC);
    out.extend_from_slice(&epoch.to_be_bytes());
    out.extend_from_slice(&(count as u16).to_be_bytes());
    out.push(depth as u8);
    for e in &entries {
        out.extend_from_slice(&e.measurement);
        out.extend_from_slice(&e.caps.to_be_bytes());
    }
    for i in 0..count {
        for node in set.path(i)? {
            out.extend_from_slice(&digest_to_bytes(node));
        }
    }
    out.extend_from_slice(&serialize_proof_ext(&proof));
    Some(out)
}
