// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use nonos_attestation_circuit::compute_capsule_commitment;

pub fn binding(
    public_inputs: &[u8],
    body_hash: Option<[u8; 32]>,
    trailer_commitment: Option<[u8; 32]>,
) -> Result<(), String> {
    let capsule = [&public_inputs[16..32], &public_inputs[48..64]].concat();
    let program = [&public_inputs[80..96], &public_inputs[112..128]].concat();
    let caps: [u8; 8] = public_inputs[152..160].try_into().map_err(|_| "caps")?;
    let commitment = [&public_inputs[176..192], &public_inputs[208..224]].concat();
    let expected =
        compute_capsule_commitment(&[capsule.as_slice(), program.as_slice(), &caps].concat());
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
