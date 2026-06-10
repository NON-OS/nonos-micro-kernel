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

use super::super::replay::{get_boot_nonce_checked, get_machine_id_checked, ZkPublicInputs};
use super::error::BindingError;

const MAX_PROOF_AGE_SECONDS: u64 = 300;

pub fn verify_proof_binding(
    public_inputs: &ZkPublicInputs,
    actual_kernel_hash: &[u8; 32],
    current_timestamp: u64,
) -> Result<(), BindingError> {
    verify_kernel_hash(public_inputs, actual_kernel_hash)?;
    verify_boot_nonce(public_inputs)?;
    verify_machine_id(public_inputs)?;
    verify_timestamp(public_inputs, current_timestamp)?;
    Ok(())
}

fn verify_kernel_hash(pi: &ZkPublicInputs, actual: &[u8; 32]) -> Result<(), BindingError> {
    if !ct_eq32(&pi.kernel_hash, actual) {
        return Err(BindingError::KernelHashMismatch);
    }
    Ok(())
}

fn verify_boot_nonce(pi: &ZkPublicInputs) -> Result<(), BindingError> {
    let current = get_boot_nonce_checked().ok_or(BindingError::NonceNotInitialized)?;
    if !ct_eq32(&pi.boot_nonce, &current) {
        return Err(BindingError::NonceMismatch);
    }
    Ok(())
}

fn verify_machine_id(pi: &ZkPublicInputs) -> Result<(), BindingError> {
    let current = get_machine_id_checked().ok_or(BindingError::MachineIdNotInitialized)?;
    if !ct_eq32(&pi.machine_id, &current) {
        return Err(BindingError::MachineIdMismatch);
    }
    Ok(())
}

fn verify_timestamp(pi: &ZkPublicInputs, current: u64) -> Result<(), BindingError> {
    let age = current.saturating_sub(pi.timestamp);
    if age > MAX_PROOF_AGE_SECONDS {
        return Err(BindingError::TimestampExpired);
    }
    Ok(())
}

#[inline]
fn ct_eq32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut x = 0u8;
    for i in 0..32 {
        x |= a[i] ^ b[i];
    }
    x == 0
}
