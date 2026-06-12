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

use super::check_commitment::check_commitment;
use super::error::AttestError;
use super::field::field;
use super::join_hi_lo::join_hi_lo;
use super::layout::{
    FE, FI_CAPSULE_HASH_HI, FI_CAP_MASK, FI_COMMITMENT_HI, FI_POLICY_EPOCH, FI_POLICY_ROOT,
    POLICY_EPOCH,
};
use super::policy_root;
use super::trailer::parse;

#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub fn verify_capsule_attestation(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
) -> Result<(), AttestError> {
    let t = parse(trailer)?;

    verify_attestation(t.proof, t.public_inputs).map_err(|_| AttestError::ProofInvalid)?;

    let capsule_hash =
        join_hi_lo(t.public_inputs, FI_CAPSULE_HASH_HI).ok_or(AttestError::Malformed)?;
    let actual = blake3::hash(elf);
    if &capsule_hash != actual.as_bytes() {
        return Err(AttestError::HashMismatch);
    }

    let policy_fe = field(t.public_inputs, FI_POLICY_ROOT).ok_or(AttestError::Malformed)?;
    let expected_policy = policy_root::root().ok_or(AttestError::PolicyRootMismatch)?;
    if policy_fe != expected_policy {
        return Err(AttestError::PolicyRootMismatch);
    }

    let epoch_fe = field(t.public_inputs, FI_POLICY_EPOCH).ok_or(AttestError::Malformed)?;
    let mut epoch_be = [0u8; FE];
    epoch_be[24..32].copy_from_slice(&POLICY_EPOCH.to_be_bytes());
    if epoch_fe != epoch_be {
        return Err(AttestError::PolicyEpochMismatch);
    }

    let cap_fe = field(t.public_inputs, FI_CAP_MASK).ok_or(AttestError::Malformed)?;
    let mut cap_be = [0u8; FE];
    cap_be[24..32].copy_from_slice(&granted_caps.to_be_bytes());
    if cap_fe != cap_be {
        return Err(AttestError::CapabilityMismatch);
    }

    let commitment_pi =
        join_hi_lo(t.public_inputs, FI_COMMITMENT_HI).ok_or(AttestError::Malformed)?;
    check_commitment(&capsule_hash, &policy_fe, granted_caps, &t.commitment, &commitment_pi)?;

    Ok(())
}
