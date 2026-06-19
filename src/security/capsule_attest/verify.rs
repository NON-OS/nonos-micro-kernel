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

use crate::crypto::zk_kernel::verify_enrolled;

use super::error::AttestError;
use super::layout::POLICY_EPOCH;
use super::policy_root;
use super::trailer::parse;

#[must_use = "a capsule must not be spawned unless its attestation verifies"]
pub fn verify_capsule_attestation(
    trailer: &[u8],
    elf: &[u8],
    granted_caps: u64,
) -> Result<(), AttestError> {
    let proof = parse(trailer)?;
    let root = policy_root::root().ok_or(AttestError::RootUnavailable)?;

    let capsule_hash = *blake3::hash(elf).as_bytes();
    let mut ctx = [0u8; 48];
    ctx[..32].copy_from_slice(&capsule_hash);
    ctx[32..40].copy_from_slice(&granted_caps.to_be_bytes());
    ctx[40..48].copy_from_slice(&POLICY_EPOCH.to_be_bytes());

    if verify_enrolled(&proof, &root, &ctx) {
        Ok(())
    } else {
        Err(AttestError::Rejected)
    }
}
