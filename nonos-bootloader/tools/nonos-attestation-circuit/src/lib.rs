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

pub mod ceremony;
pub mod circuit;
pub mod constants;
pub mod nox;
pub mod policy_tree;
pub mod verify;

pub use ceremony::{
    add_destruction_attestation, ceremony_finalize, ceremony_init, contribute_randomness,
    verify_contribution, CeremonyError, CeremonyMetadata, CeremonyParams, CeremonyTranscript,
    ContributionRecord, DestructionAttestation, MIN_PARTICIPANTS,
};
pub use circuit::NonosAttestationCircuit;
pub use constants::{
    compute_build_config_hash, compute_capsule_commitment, compute_cargo_lock_hash,
    compute_rustc_version_hash, compute_source_tree_hash, expected_program_hash_bytes,
    BuildProvenance, BUILD_PROVENANCE_HASH_COUNT, DS_BUILD_CONFIG, DS_BUILD_PROVENANCE,
    DS_CARGO_LOCK, DS_COMMITMENT, DS_PROGRAM, DS_RUSTC_VERSION, DS_SOURCE_TREE, GROTH16_PROOF_SIZE,
    MIN_HW_LEVEL, MIN_PCR_ENTROPY_BYTES, PCR_PREIMAGE_LEN,
};
pub use policy_tree::POLICY_EPOCH;
