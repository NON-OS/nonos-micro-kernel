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

use super::detect::find_zk_proof_offset;
use super::types::{
    ZkProofBlock, RUNTIME_BOOT_PUBLIC_INPUTS_LEN, STATIC_BOOT_PUBLIC_INPUTS_LEN,
    TRANSPARENT_MIN_PROOF_SIZE, ZK_PROOF_HEADER_SIZE, ZK_PROOF_MAGIC, ZK_PROOF_VERSION,
};

pub fn parse_zk_proof(kernel_data: &[u8]) -> Result<(ZkProofBlock, usize), &'static str> {
    let offset = find_zk_proof_offset(kernel_data).ok_or("ZK proof magic not found")?;
    let block = &kernel_data[offset..];

    if block.len() < ZK_PROOF_HEADER_SIZE {
        return Err("ZK proof block too small");
    }

    if &block[0..4] != &ZK_PROOF_MAGIC {
        return Err("ZK proof magic mismatch");
    }

    let version = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
    if version != ZK_PROOF_VERSION {
        return Err("unsupported ZK proof version");
    }

    let mut program_hash = [0u8; 32];
    program_hash.copy_from_slice(&block[8..40]);

    let mut capsule_commitment = [0u8; 32];
    capsule_commitment.copy_from_slice(&block[40..72]);

    let mut kernel_hash = [0u8; 32];
    kernel_hash.copy_from_slice(&block[72..104]);

    let mut boot_nonce = [0u8; 32];
    boot_nonce.copy_from_slice(&block[104..136]);

    let mut machine_id = [0u8; 32];
    machine_id.copy_from_slice(&block[136..168]);

    let public_inputs_len =
        u32::from_le_bytes([block[168], block[169], block[170], block[171]]) as usize;
    let proof_blob_len =
        u32::from_le_bytes([block[172], block[173], block[174], block[175]]) as usize;

    if public_inputs_len > 256 * 1024 {
        return Err("public inputs too large");
    }
    if proof_blob_len < TRANSPARENT_MIN_PROOF_SIZE {
        return Err("transparent proof too small");
    }
    if public_inputs_len != STATIC_BOOT_PUBLIC_INPUTS_LEN
        && public_inputs_len != RUNTIME_BOOT_PUBLIC_INPUTS_LEN
    {
        return Err("transparent public inputs size invalid");
    }

    let data_start = ZK_PROOF_HEADER_SIZE;
    let required_len = data_start + public_inputs_len + proof_blob_len;
    if block.len() < required_len {
        return Err("ZK proof block truncated");
    }

    let public_inputs = block[data_start..data_start + public_inputs_len].to_vec();
    let proof_blob = block
        [data_start + public_inputs_len..data_start + public_inputs_len + proof_blob_len]
        .to_vec();
    if &public_inputs[0..32] != kernel_hash.as_slice() {
        return Err("kernel hash public input mismatch");
    }
    if public_inputs_len == RUNTIME_BOOT_PUBLIC_INPUTS_LEN {
        if &public_inputs[32..64] != boot_nonce.as_slice() {
            return Err("boot nonce public input mismatch");
        }
        if &public_inputs[72..104] != machine_id.as_slice() {
            return Err("machine id public input mismatch");
        }
    }

    Ok((
        ZkProofBlock {
            program_hash,
            capsule_commitment,
            kernel_hash,
            boot_nonce,
            machine_id,
            public_inputs,
            proof_blob,
        },
        offset,
    ))
}
