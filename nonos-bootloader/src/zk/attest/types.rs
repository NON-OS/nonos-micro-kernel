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

use crate::zk::verify::ZkVerifyResult;
use alloc::vec::Vec;

pub const ZK_PROOF_MAGIC: [u8; 4] = [0x4E, 0xC3, 0x5A, 0x50];

pub const ZK_PROOF_VERSION: u32 = 2;

pub const ZK_PROOF_HEADER_SIZE: usize = 176;

pub const TRANSPARENT_MIN_PROOF_SIZE: usize = 129;
pub const STATIC_BOOT_PUBLIC_INPUTS_LEN: usize = 32;
pub const RUNTIME_BOOT_PUBLIC_INPUTS_LEN: usize = 104;

#[derive(Debug, Clone)]
pub struct BootAttestationResult {
    pub zk_verified: bool,
    pub program_hash: [u8; 32],
    pub capsule_commitment: [u8; 32],
    pub detail: ZkVerifyResult,
    pub status_message: &'static str,
}

impl Default for BootAttestationResult {
    fn default() -> Self {
        Self {
            zk_verified: false,
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            detail: ZkVerifyResult::Unsupported("not verified"),
            status_message: "not verified",
        }
    }
}

impl BootAttestationResult {
    /// Create result for when no proof is present
    pub fn no_proof() -> Self {
        Self {
            zk_verified: false,
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            detail: ZkVerifyResult::Unsupported("no ZK proof present"),
            status_message: "no ZK proof present in capsule",
        }
    }

    /// Create result for parse error
    pub fn parse_error(msg: &'static str) -> Self {
        Self {
            zk_verified: false,
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            detail: ZkVerifyResult::Error(msg),
            status_message: msg,
        }
    }

    /// Create result for verification failure
    pub fn verification_failed(
        program_hash: [u8; 32],
        capsule_commitment: [u8; 32],
        msg: &'static str,
    ) -> Self {
        Self {
            zk_verified: false,
            program_hash,
            capsule_commitment,
            detail: ZkVerifyResult::Invalid(msg),
            status_message: msg,
        }
    }

    /// Create result for successful verification
    pub fn verified(program_hash: [u8; 32], capsule_commitment: [u8; 32]) -> Self {
        Self {
            zk_verified: true,
            program_hash,
            capsule_commitment,
            detail: ZkVerifyResult::Valid,
            status_message: "ZK attestation verified",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ZkProofBlock {
    pub program_hash: [u8; 32],
    pub capsule_commitment: [u8; 32],
    pub kernel_hash: [u8; 32],
    pub boot_nonce: [u8; 32],
    pub machine_id: [u8; 32],
    pub public_inputs: Vec<u8>,
    pub proof_blob: Vec<u8>,
}

impl ZkProofBlock {
    pub fn is_valid(&self) -> bool {
        (self.public_inputs.len() == STATIC_BOOT_PUBLIC_INPUTS_LEN
            || self.public_inputs.len() == RUNTIME_BOOT_PUBLIC_INPUTS_LEN)
            && self.proof_blob.len() >= TRANSPARENT_MIN_PROOF_SIZE
    }

    pub fn kernel_hash_matches(&self, actual: &[u8; 32]) -> bool {
        ct_eq32(&self.kernel_hash, actual)
    }

    pub fn is_runtime(&self) -> bool {
        self.public_inputs.len() == RUNTIME_BOOT_PUBLIC_INPUTS_LEN
    }
}

#[inline]
fn ct_eq32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut x = 0u8;
    for i in 0..32 {
        x |= a[i] ^ b[i];
    }
    x == 0
}
