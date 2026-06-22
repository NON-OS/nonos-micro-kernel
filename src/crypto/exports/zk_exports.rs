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

pub use super::super::zk::nonos_zk;

#[cfg(feature = "zk-halo2")]
pub use super::super::zk::halo2::{halo2_verify, Halo2Error, Halo2Verifier};

pub use super::super::zk::nonos_zk::{
    commit, commit_u64, create_attestation, issue_credential, verify_attestation,
    verify_commitment, verify_credential, zeroize_array, zeroize_mut, AttestationProof, Credential,
};
pub use super::super::zk_kernel::{
    plonk_prove, plonk_verify, syscall_zk_commit, syscall_zk_prove_plonk, syscall_zk_verify,
    zeroize as zk_zeroize,
};
pub use super::super::zk_kernel::{
    EqualityProof, FieldElement, KernelZkVerifier, MembershipProof, PedersenCommitment,
    PlonkCircuit, PlonkEvaluations, PlonkProof, ProofSystem, ZkError, ZkResult, KERNEL_ZK_VERIFIER,
};
