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
//! so is the leaf: the gate measures the ELF it is about to run, and the proof must
//! open that measurement. Verified at extension-field soundness before a spawn is
//! allowed.

use super::error::AttestError;
use super::layout::{POLICY_EPOCH, POLICY_TREE_DEPTH};
use crate::crypto::stark::air::{verify_membership_trailer, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
// One definition, in crate::crypto::stark. Prover and verifier must
// agree exactly; a drift downward in queries or grinding still verifies.
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};

/// Verify a capsule's transparent-STARK attestation against `policy`, bound to
/// its measurement, its granted capabilities and the epoch. True only for a
/// money-grade proof that the measurement of exactly this ELF sits under exactly
/// this root.
///
/// The root is a parameter rather than a lookup, so a capsule built on this
/// machine clears exactly the bar a shipped one does. Only whose tree it is
/// proved against differs.
///
/// The trailer parse is the crate's, not a copy of it. The gate, the boot chain
/// and the enrollment tool read one layout from one place.
#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub(super) fn verify_against(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
    policy: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    // Bind the proof to the capsule: its measurement, its capabilities, the epoch.
    let capsule_hash = *blake3::hash(elf).as_bytes();
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(&capsule_hash);
    ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());

    let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    if verify_membership_trailer(
        &hasher,
        LOG_ROUNDS,
        *policy,
        elf,
        POLICY_TREE_DEPTH,
        trailer,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    ) {
        Ok(capsule_hash)
    } else {
        Err(AttestError::Rejected)
    }
}
