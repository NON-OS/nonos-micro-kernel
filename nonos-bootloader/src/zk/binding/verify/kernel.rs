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

use super::super::replay::ZkPublicInputs;
use super::error::BindingError;

pub fn verify_kernel_binding(
    public_inputs: &ZkPublicInputs,
    actual_kernel_hash: &[u8; 32],
) -> Result<(), BindingError> {
    if !ct_eq32(&public_inputs.kernel_hash, actual_kernel_hash) {
        return Err(BindingError::KernelHashMismatch);
    }
    Ok(())
}

pub fn extract_kernel_hash_from_inputs(raw_inputs: &[u8]) -> Result<[u8; 32], BindingError> {
    if raw_inputs.len() < 32 {
        return Err(BindingError::PublicInputsMalformed);
    }
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&raw_inputs[0..32]);
    Ok(hash)
}

#[inline]
fn ct_eq32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut x = 0u8;
    for i in 0..32 {
        x |= a[i] ^ b[i];
    }
    x == 0
}
