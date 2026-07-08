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

//! A post-quantum capsule attestation: a context-bound STARK proof that the
//! prover knows an enrolled secret. The enrolled leaf, the secret, stays
//! private; the Merkle path to the policy root is public, exactly as the
//! Curve25519 gate reveals its siblings while hiding the committed secret.
//! Verification recomputes nothing about the secret: it rebuilds the membership
//! AIR with the kernel's own trusted root and the supplied path, then checks the
//! proof under the capsule context, so a proof made for one capsule cannot admit
//! another and a proof against a different root does not verify. It reads
//! attacker-supplied proof bytes and never panics.

use super::super::field::{Fp, P};
use super::poseidon::{Poseidon, RATE};
use super::serialize::deserialize_proof;
use super::verify::stark_verify_bound;
use super::MerkleMembership;
use alloc::vec::Vec;

/// The magic that tags a STARK attestation trailer, distinct from the
/// Curve25519 trailer's tag so the gate can route on it.
pub const STARK_ATTEST_MAGIC: &[u8; 8] = b"NZKSTRK1";

/// Verify a context-bound membership attestation. `root` is the kernel's trusted
/// Poseidon policy root, never the prover's; `siblings` and `directions` are the
/// public path the proof commits to; `context` binds the proof to the capsule
/// being spawned. Returns true only if the serialized proof is a valid
/// membership proof under exactly this root and context.
#[must_use = "an attestation result must gate the spawn"]
#[allow(clippy::too_many_arguments)]
pub fn verify_membership_attestation(
    hasher: &Poseidon,
    log_rounds: u32,
    root: [Fp; RATE],
    siblings: &[[Fp; RATE]],
    directions: &[bool],
    n_queries: usize,
    proof_bytes: &[u8],
    context: &[u8],
) -> bool {
    // A path must be non-empty and its two halves must agree in length; the AIR
    // reads both, so a mismatch is a malformed attestation.
    if siblings.is_empty() || siblings.len() != directions.len() {
        return false;
    }
    let proof = match deserialize_proof(proof_bytes) {
        Some(p) => p,
        None => return false,
    };
    let air = MerkleMembership::new(
        hasher.clone(),
        log_rounds,
        root,
        siblings.to_vec(),
        directions.to_vec(),
    );
    stark_verify_bound(&air, &proof, n_queries, context)
}

/// Read a field element from eight canonical little-endian bytes, rejecting a
/// non-canonical value.
fn read_fp(b: &[u8]) -> Option<Fp> {
    if b.len() != 8 {
        return None;
    }
    let v = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
    if v >= P {
        return None;
    }
    Some(Fp::from_u64(v))
}

/// Verify a whole STARK attestation trailer against the kernel's trusted policy
/// root and capsule context. The trailer carries only the public Merkle path and
/// the proof; the round count and query count are the kernel's own constants,
/// never read from the trailer, so a prover cannot weaken the low-degree test by
/// claiming fewer queries. Layout after the magic: one depth byte, the direction
/// bits packed one per level, `depth * RATE` field elements of siblings, then the
/// serialized proof. Total over any bytes: it never panics.
///
/// `blob` is the whole trailer including the magic. Returns true only for a valid
/// membership proof, bound to this context, under this root.
#[must_use = "an attestation result must gate the spawn"]
pub fn verify_attestation_trailer(
    hasher: &Poseidon,
    log_rounds: u32,
    root: [Fp; RATE],
    n_queries: usize,
    blob: &[u8],
    context: &[u8],
) -> bool {
    // Magic.
    if blob.len() < 9 || &blob[0..8] != STARK_ATTEST_MAGIC {
        return false;
    }
    let depth = blob[8] as usize;
    if depth == 0 {
        return false;
    }
    let dir_bytes = depth.div_ceil(8);
    let sib_bytes = depth * RATE * 8;
    let header = 9 + dir_bytes + sib_bytes;
    if blob.len() < header {
        return false;
    }

    // Directions: bit `i` is the index bit at level `i`.
    let dir_slice = &blob[9..9 + dir_bytes];
    let directions: Vec<bool> =
        (0..depth).map(|i| (dir_slice[i / 8] >> (i % 8)) & 1 == 1).collect();

    // Siblings: `depth` digests of `RATE` canonical field elements each.
    let sib_slice = &blob[9 + dir_bytes..header];
    let mut siblings: Vec<[Fp; RATE]> = Vec::with_capacity(depth);
    for level in 0..depth {
        let mut sib = [Fp::ZERO; RATE];
        for (c, cell) in sib.iter_mut().enumerate() {
            let off = (level * RATE + c) * 8;
            match read_fp(&sib_slice[off..off + 8]) {
                Some(v) => *cell = v,
                None => return false,
            }
        }
        siblings.push(sib);
    }

    let proof_bytes = &blob[header..];
    verify_membership_attestation(
        hasher,
        log_rounds,
        root,
        &siblings,
        &directions,
        n_queries,
        proof_bytes,
        context,
    )
}
