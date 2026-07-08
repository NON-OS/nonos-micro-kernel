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

//! Post-quantum capsule attestation: a context-bound STARK proof that the
//! prover knows an enrolled secret. The leaf stays private, the path is public,
//! as the Curve25519 gate hides its secret and reveals its siblings.

use super::super::field::Fp;
use super::deserialize::deserialize_proof;
use super::poseidon::{Poseidon, RATE};
use super::verify::stark_verify_bound;
use super::MerkleMembership;

/// Verify a context-bound membership attestation. `root` is the kernel's own
/// trusted policy root, never the prover's; `context` binds the proof to the
/// capsule. True only for a valid proof under exactly this root and context.
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
    if siblings.is_empty() || siblings.len() != directions.len() {
        return false;
    }
    let Some(proof) = deserialize_proof(proof_bytes) else {
        return false;
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
