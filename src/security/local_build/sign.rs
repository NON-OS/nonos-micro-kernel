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

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::zk_kernel::prove_enrolled;
use crate::security::capsule_attest::layout::POLICY_EPOCH;

use super::error::LocalBuildError;
use super::identity::with_identity;
use super::trailer::encode;

/// Laid out as `against_root::verify` lays it out. If the two disagree the
/// proof verifies against nothing.
fn context(elf: &[u8], granted_caps: u64) -> [u8; 48] {
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(blake3::hash(elf).as_bytes());
    ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());
    ctx
}

/// Prove this machine may run `elf` holding `granted_caps`.
pub fn sign(elf: &[u8], granted_caps: u64) -> Result<Vec<u8>, LocalBuildError> {
    /*
     * This used to refuse under `nonos-stark-attest`, because the verifier
     * picked its parser from that flag and would have read these NZKCAPS2
     * bytes as a malformed STARK.
     */
    let ctx = context(elf, granted_caps);
    let proof = with_identity(|id| {
        prove_enrolled(&id.secret, &id.blinding, 0, &super::tree::empty_siblings(), &id.root, &ctx)
    })
    .ok_or(LocalBuildError::NoIdentity)?
    .ok_or(LocalBuildError::ProofFailed)?;
    encode(&proof).ok_or(LocalBuildError::TrailerShape)
}
