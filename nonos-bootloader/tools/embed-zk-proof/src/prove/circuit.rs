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

use ark_bls12_381::Fr;
use nonos_attestation_circuit::{
    compute_capsule_commitment, expected_program_hash_bytes, policy_tree, NonosAttestationCircuit,
    MIN_HW_LEVEL, PCR_PREIMAGE_LEN, POLICY_EPOCH,
};

pub struct CircuitParams {
    pub kernel_hash: [u8; 32],
    pub boot_nonce: [u8; 32],
    pub machine_id: [u8; 32],
    pub capsule_commitment: [u8; 32],
    pub program_hash: [u8; 32],
    pub pcr_preimage: [u8; PCR_PREIMAGE_LEN],
    pub hardware_attestation: u64,
}

pub fn create_circuit_params(
    kernel_bytes: &[u8],
    seed: &str,
    boot_nonce: &[u8; 32],
    machine_id: &[u8; 32],
) -> CircuitParams {
    let program_hash = expected_program_hash_bytes();
    let kernel_hash = *blake3::hash(kernel_bytes).as_bytes();

    let mut pcr_preimage = [0u8; PCR_PREIMAGE_LEN];
    let mut hasher = blake3::Hasher::new();
    hasher.update(seed.as_bytes());
    hasher.update(&kernel_hash);
    hasher.update(b"pcr_preimage_v1");
    let hash = hasher.finalize();
    pcr_preimage[..32].copy_from_slice(hash.as_bytes());
    pcr_preimage[32..].copy_from_slice(hash.as_bytes());

    let mut public_input_seed = Vec::with_capacity(72);
    public_input_seed.extend_from_slice(&kernel_hash);
    public_input_seed.extend_from_slice(&program_hash);
    public_input_seed.extend_from_slice(&0u64.to_be_bytes());
    let capsule_commitment = compute_capsule_commitment(&public_input_seed);

    CircuitParams {
        kernel_hash,
        boot_nonce: *boot_nonce,
        machine_id: *machine_id,
        capsule_commitment,
        program_hash,
        pcr_preimage,
        hardware_attestation: MIN_HW_LEVEL + 0x2000,
    }
}

pub fn build_circuit(params: &CircuitParams) -> Result<NonosAttestationCircuit<Fr>, String> {
    let (hi, lo) = policy_tree::split_hash(&params.kernel_hash);
    let policy = policy_tree::witness(&[(hi, lo, 0)], 0)?;
    Ok(NonosAttestationCircuit::new(
        params.kernel_hash,
        policy,
        POLICY_EPOCH,
        0,
        params.capsule_commitment,
        params.pcr_preimage,
        params.hardware_attestation,
    ))
}
