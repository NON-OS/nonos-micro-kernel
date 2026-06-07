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

use crate::crypto::zk::groth16::verify_attestation;

use super::error::AttestError;
use super::layout::{fe, join_hi_lo, FE, FI_CAPSULE_HASH_HI, FI_CAP_MASK, FI_COMMITMENT_HI};
use super::trailer::parse;

// Verify a capsule's embedded attestation. `trailer` is the capsule's ZK trailer
// bytes, `elf` the capsule payload about to run, and `granted_caps` the
// capability mask the kernel is about to install. Returns Ok(()) only when the
// proof is cryptographically valid AND binds to exactly these bytes and caps.
//
// The kernel does not recompute the commitment hash, so this stays independent
// of the proving side's commitment scheme. Soundness comes from the proof plus
// the two reality bindings below.
#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub fn verify_capsule_attestation(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
) -> Result<(), AttestError> {
    let t = parse(trailer)?;

    // 1. Cryptographic verification against the kernel's embedded verifying key.
    //    The proof binds all public inputs, including the commitment.
    verify_attestation(t.proof, t.public_inputs).map_err(|_| AttestError::ProofInvalid)?;

    // 2. Bind to the real capsule bytes: the capsule hash in the proof must
    //    equal blake3 of the payload that is about to be mapped.
    let capsule_hash =
        join_hi_lo(t.public_inputs, FI_CAPSULE_HASH_HI).ok_or(AttestError::Malformed)?;
    let actual = blake3::hash(elf);
    if &capsule_hash != actual.as_bytes() {
        return Err(AttestError::HashMismatch);
    }

    // 3. Bind to policy: the capability mask in the proof must equal the grant.
    let cap_fe = fe(t.public_inputs, FI_CAP_MASK).ok_or(AttestError::Malformed)?;
    let mut cap_be = [0u8; FE];
    cap_be[24..32].copy_from_slice(&granted_caps.to_be_bytes());
    if cap_fe != cap_be {
        return Err(AttestError::CapabilityMismatch);
    }

    // 4. Trailer integrity: the commitment carried in the trailer must equal the
    //    commitment the proof binds in its public inputs.
    let commitment_pi =
        join_hi_lo(t.public_inputs, FI_COMMITMENT_HI).ok_or(AttestError::Malformed)?;
    if t.commitment != commitment_pi {
        return Err(AttestError::CommitmentMismatch);
    }

    Ok(())
}
