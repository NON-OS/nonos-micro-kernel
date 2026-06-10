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

use crate::constants::compute_capsule_commitment;

pub fn binding(
    public_inputs: &[u8],
    body_hash: Option<[u8; 32]>,
    trailer_commitment: Option<[u8; 32]>,
) -> Result<(), String> {
    let hi = public_inputs.get(16..32).ok_or("capsule hash high")?;
    let lo = public_inputs.get(48..64).ok_or("capsule hash low")?;
    let capsule = [hi, lo].concat();
    let policy_root = public_inputs.get(64..96).ok_or("policy root")?;
    let epoch = public_inputs.get(120..128).ok_or("policy epoch")?;
    let caps = public_inputs.get(152..160).ok_or("capability mask")?;
    let commit_hi = public_inputs.get(176..192).ok_or("commitment high")?;
    let commit_lo = public_inputs.get(208..224).ok_or("commitment low")?;
    let commitment = [commit_hi, commit_lo].concat();
    let expected =
        compute_capsule_commitment(&[capsule.as_slice(), policy_root, epoch, caps].concat());
    if let Some(hash) = body_hash {
        if capsule.as_slice() != hash.as_slice() {
            return Err("capsule hash mismatch".into());
        }
    }
    if expected.as_slice() != commitment.as_slice() {
        return Err("commitment mismatch".into());
    }
    if let Some(hash) = trailer_commitment {
        if commitment.as_slice() != hash.as_slice() {
            return Err("trailer commitment mismatch".into());
        }
    }
    Ok(())
}
