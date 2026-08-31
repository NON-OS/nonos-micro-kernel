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

//! Zero-knowledge proof primitives for the NONOS kernel.
//!
//! Provides:
//! - Pedersen commitments
//! - Range proofs
//! - Equality proofs
//! - Merkle membership proofs
//! - PLONK proof system
//! - Kernel-level ZK verifier

extern crate alloc;

mod attest;
pub(crate) mod constants;
mod equality;
mod field;
mod membership;
mod pedersen;
mod plonk;
mod range;
mod syscall;
mod utils;
mod verifier;

// Re-export constants
pub use constants::{DOM_EQUALITY, DOM_MERKLE, DOM_PEDERSEN, DOM_PLONK, DOM_RANGE, L};

// Re-export field element
pub use field::FieldElement;

// Re-export utility functions
pub use utils::{constant_time_eq, zeroize};

// Re-export proof types
pub use attest::{prove_enrolled, verify_enrolled, EnrolledSecretProof};
pub use equality::EqualityProof;
pub use membership::MembershipProof;
pub use pedersen::PedersenCommitment;
pub use plonk::{plonk_prove, plonk_verify, PlonkCircuit, PlonkEvaluations, PlonkProof};

// Re-export verifier
pub use verifier::{KernelZkVerifier, ProofSystem, ZkResult, KERNEL_ZK_VERIFIER};

// Re-export syscall interface
pub use syscall::{
    syscall_zk_commit, syscall_zk_prove_plonk, syscall_zk_prove_range, syscall_zk_verify, ZkError,
};
