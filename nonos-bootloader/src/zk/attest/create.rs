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

extern crate alloc;

use crate::zk::binding::DS_COMMITMENT;
use alloc::vec::Vec;

use super::types::{
    RUNTIME_BOOT_PUBLIC_INPUTS_LEN, STATIC_BOOT_PUBLIC_INPUTS_LEN, TRANSPARENT_MIN_PROOF_SIZE,
    ZK_PROOF_HEADER_SIZE, ZK_PROOF_MAGIC, ZK_PROOF_VERSION,
};

/// Compute capsule commitment from kernel code
pub fn compute_capsule_commitment(kernel_code: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_COMMITMENT);
    hasher.update(kernel_code);
    *hasher.finalize().as_bytes()
}

/// Create a ZK proof block for embedding in capsule
pub fn create_zk_proof_block(
    program_hash: &[u8; 32],
    capsule_commitment: &[u8; 32],
    public_inputs: &[u8],
    proof_blob: &[u8],
) -> Result<Vec<u8>, &'static str> {
    if proof_blob.len() < TRANSPARENT_MIN_PROOF_SIZE {
        return Err("transparent proof too small");
    }

    if public_inputs.len() != STATIC_BOOT_PUBLIC_INPUTS_LEN
        && public_inputs.len() != RUNTIME_BOOT_PUBLIC_INPUTS_LEN
    {
        return Err("transparent public inputs size invalid");
    }

    let mut block =
        Vec::with_capacity(ZK_PROOF_HEADER_SIZE + public_inputs.len() + proof_blob.len());

    block.extend_from_slice(&ZK_PROOF_MAGIC);
    block.extend_from_slice(&ZK_PROOF_VERSION.to_le_bytes());
    block.extend_from_slice(program_hash);
    block.extend_from_slice(capsule_commitment);
    block.extend_from_slice(&public_inputs[0..32]);
    if public_inputs.len() == RUNTIME_BOOT_PUBLIC_INPUTS_LEN {
        block.extend_from_slice(&public_inputs[32..64]);
        block.extend_from_slice(&public_inputs[72..104]);
    } else {
        block.extend_from_slice(&[0u8; 64]);
    }
    block.extend_from_slice(&(public_inputs.len() as u32).to_le_bytes());
    block.extend_from_slice(&(proof_blob.len() as u32).to_le_bytes());

    block.extend_from_slice(public_inputs);
    block.extend_from_slice(proof_blob);

    Ok(block)
}

/// Calculate total size of ZK proof block
pub fn calculate_proof_block_size(public_inputs_len: usize) -> usize {
    ZK_PROOF_HEADER_SIZE + public_inputs_len + TRANSPARENT_MIN_PROOF_SIZE
}
