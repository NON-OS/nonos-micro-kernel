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

pub mod attest;
pub mod binding;
pub mod errors;
pub mod registry;
pub mod section;
pub mod transcript;
pub mod verify;

pub use errors::ZkError;

pub use binding::{
    compute_capsule_commitment, compute_commit, is_manifest_binding_enabled, select_binding,
    verify_commitment, verify_kernel_binding, verify_proof_binding, BindingError, BindingInput,
    DS_COMMITMENT, MAX_MANIFEST_SIZE,
};

pub use binding::replay::{
    build_public_inputs, derive_machine_id, get_boot_nonce, get_machine_id, init_boot_nonce,
    init_machine_id, is_machine_id_initialized, is_nonce_initialized, verify_machine_id,
    verify_nonce_freshness, ZkPublicInputs,
};

pub use verify::{ct_eq32, derive_program_hash, verify_proof, ZkProof, ZkVerifyResult};
#[cfg(feature = "zk-groth16")]
pub use verify::{groth16_verify, GrothErr, GROTH16_PROOF_LEN};
pub use verify::{DS_PROGRAM_HASH, MAX_INPUT_SIZE, MAX_PROOF_SIZE};

pub use attest::{
    calculate_proof_block_size, create_zk_proof_block, find_zk_proof_offset, has_zk_proof,
    parse_zk_proof, verify_boot_attestation, verify_boot_attestation_with_manifest,
    BootAttestationResult, ZkProofBlock, GROTH16_PROOF_SIZE, ZK_PROOF_HEADER_SIZE, ZK_PROOF_MAGIC,
    ZK_PROOF_VERSION,
};

pub use transcript::{Transcript, TRANSCRIPT_DOMAIN_BOOT, TRANSCRIPT_DOMAIN_CIRCUIT};

pub use section::{parse_section, validate_section};

pub use registry::{derive_circuit_key, verify_circuit_key_derivation};
pub use registry::{
    parse_circuit_section, CircuitCategory, CircuitEntry, CircuitPermission, CircuitSectionEntry,
    CircuitSectionHeader, DynamicCircuitEntry, DynamicCircuitStore, CIRCUIT_SECTION_MAGIC,
};

#[cfg(feature = "zk-groth16")]
pub use registry::{
    circuits_with_permission, core_circuits, has_permission, lookup, lookup_circuit,
    vk_boot_authority, vk_recovery_key, vk_update_authority, PROGRAM_HASH_BOOT_AUTHORITY,
    PROGRAM_HASH_RECOVERY_KEY, PROGRAM_HASH_UPDATE_AUTHORITY,
};
