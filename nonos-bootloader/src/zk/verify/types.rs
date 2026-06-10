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

use alloc::vec::Vec;

/// _* ZK proof structure for verification *_
#[derive(Debug, Clone)]
pub struct ZkProof {
    pub program_hash: [u8; 32],
    pub capsule_commitment: [u8; 32],
    pub public_inputs: Vec<u8>,
    pub proof_blob: Vec<u8>,
    /// Serialized proof blob (192 bytes for Groth16)
    pub manifest: Option<Vec<u8>>,
}

impl ZkProof {
    pub fn new() -> Self {
        Self {
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            public_inputs: Vec::new(),
            proof_blob: Vec::new(),
            manifest: None,
        }
    }

    pub fn has_valid_input_alignment(&self) -> bool {
        self.public_inputs.len() % 32 == 0
    }

    pub fn input_count(&self) -> usize {
        self.public_inputs.len() / 32
    }
}

impl Default for ZkProof {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ZkVerifyResult {
    Valid,
    Invalid(&'static str),
    Unsupported(&'static str),
    Error(&'static str),
}

impl ZkVerifyResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ZkVerifyResult::Valid)
    }

    pub fn error_message(&self) -> Option<&'static str> {
        match self {
            ZkVerifyResult::Valid => None,
            ZkVerifyResult::Invalid(msg) => Some(msg),
            ZkVerifyResult::Unsupported(msg) => Some(msg),
            ZkVerifyResult::Error(msg) => Some(msg),
        }
    }
}
