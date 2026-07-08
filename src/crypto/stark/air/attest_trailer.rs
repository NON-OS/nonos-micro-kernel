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

//! Parsing and verifying a whole attestation trailer, the bytes a capsule
//! carries. The round and query counts are the kernel's, never the trailer's,
//! so a prover cannot weaken the low-degree test.

use super::super::field::{Fp, P};
use super::attest::verify_membership_attestation;
use super::poseidon::{Poseidon, RATE};
use alloc::vec::Vec;

/// The tag distinguishing a STARK trailer from the Curve25519 one.
pub const STARK_ATTEST_MAGIC: &[u8; 8] = b"NZKSTRK1";

fn read_fp(b: &[u8]) -> Option<Fp> {
    let v = u64::from_le_bytes(b.try_into().ok()?);
    (v < P).then(|| Fp::from_u64(v))
}

/// Verify a trailer against the trusted `root` and capsule `context`. Layout
/// after the magic: a depth byte, the direction bits, `depth * RATE` sibling
/// field elements, then the serialized proof. Total over any bytes.
#[must_use = "an attestation result must gate the spawn"]
pub fn verify_attestation_trailer(
    hasher: &Poseidon,
    log_rounds: u32,
    root: [Fp; RATE],
    n_queries: usize,
    blob: &[u8],
    context: &[u8],
) -> bool {
    if blob.len() < 9 || &blob[0..8] != STARK_ATTEST_MAGIC {
        return false;
    }
    let depth = blob[8] as usize;
    if depth == 0 {
        return false;
    }
    let dir_bytes = depth.div_ceil(8);
    let header = 9 + dir_bytes + depth * RATE * 8;
    if blob.len() < header {
        return false;
    }

    let dir = &blob[9..9 + dir_bytes];
    let directions: Vec<bool> = (0..depth).map(|i| (dir[i / 8] >> (i % 8)) & 1 == 1).collect();

    let sib = &blob[9 + dir_bytes..header];
    let mut siblings = Vec::with_capacity(depth);
    for level in 0..depth {
        let mut d = [Fp::ZERO; RATE];
        for (c, cell) in d.iter_mut().enumerate() {
            let off = (level * RATE + c) * 8;
            match read_fp(&sib[off..off + 8]) {
                Some(v) => *cell = v,
                None => return false,
            }
        }
        siblings.push(d);
    }

    verify_membership_attestation(
        hasher,
        log_rounds,
        root,
        &siblings,
        &directions,
        n_queries,
        &blob[header..],
        context,
    )
}
