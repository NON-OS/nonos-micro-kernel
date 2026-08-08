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

//! The transparent, post-quantum spawn gate. A capsule ships a money-grade STARK
//! proof that its measurement is enrolled under the kernel policy root, bound to the
//! capsule context. The trusted root is the kernel's own, never the trailer's, and
//! the proof is verified at extension-field soundness before a spawn is allowed. This
//! replaces the forgeable pairing gate with a sound one.

use super::error::AttestError;
use super::layout::{POLICY_EPOCH, POLICY_TREE_DEPTH};
use super::policy_root;
use crate::crypto::stark::air::{
    deserialize_proof_ext, stark_verify_ext_blown_bound, MerkleMembership, Poseidon, RATE,
};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

const MAGIC: &[u8; 8] = b"NZKSTRK1";
const LOG_ROUNDS: u32 = 3;
const N_QUERIES: usize = 32;
const GRIND_BITS: u32 = 16;
const EXTRA_BLOWUP: u32 = 3;

/// Read four little-endian words into a rate-width Poseidon digest.
fn to_rate(bytes: &[u8]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    for (i, lane) in out.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    out
}

/// Verify a capsule's transparent-STARK attestation against the kernel policy
/// root, bound to its measurement, its granted capabilities and the epoch. True
/// only for a money-grade membership proof under exactly this root and context.
#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub fn verify_capsule_attestation_stark(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
) -> Result<(), AttestError> {
    let dir_bytes = POLICY_TREE_DEPTH.div_ceil(8);
    let sib_end = 9 + POLICY_TREE_DEPTH * 32;
    if trailer.len() < sib_end + dir_bytes
        || &trailer[0..8] != MAGIC
        || trailer[8] as usize != POLICY_TREE_DEPTH
    {
        return Err(AttestError::Malformed);
    }

    let mut siblings = Vec::with_capacity(POLICY_TREE_DEPTH);
    for i in 0..POLICY_TREE_DEPTH {
        siblings.push(to_rate(&trailer[9 + i * 32..9 + i * 32 + 32]));
    }
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let directions: Vec<bool> =
        (0..POLICY_TREE_DEPTH).map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1).collect();
    let proof =
        deserialize_proof_ext(&trailer[sib_end + dir_bytes..]).ok_or(AttestError::Malformed)?;

    let root = to_rate(&policy_root::root().ok_or(AttestError::RootUnavailable)?);

    // Bind the proof to the capsule: its measurement, its capabilities, the epoch.
    let capsule_hash = *blake3::hash(elf).as_bytes();
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(&capsule_hash);
    ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());

    let air = MerkleMembership::new(
        Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]),
        LOG_ROUNDS,
        root,
        siblings,
        directions,
    );
    if stark_verify_ext_blown_bound(&air, &proof, N_QUERIES, GRIND_BITS, EXTRA_BLOWUP, &ctx) {
        Ok(())
    } else {
        Err(AttestError::Rejected)
    }
}
