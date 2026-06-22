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

extern crate alloc;

use super::{KernelZkVerifier, ZkResult};
use crate::crypto::zk_kernel::range::RangeProof;
use alloc::vec::Vec;

impl KernelZkVerifier {
    pub fn verify_range(&mut self, proof_bytes: &[u8]) -> ZkResult {
        if proof_bytes.len() < 33 {
            return ZkResult::MalformedProof;
        }
        let bits = proof_bytes[0];
        let expected_len = 1 + 32 + (bits as usize * 32);
        if proof_bytes.len() < expected_len {
            return ZkResult::MalformedProof;
        }
        let mut response = [0u8; 32];
        response.copy_from_slice(&proof_bytes[1..33]);
        let mut bit_commitments = Vec::with_capacity(bits as usize);
        for i in 0..bits as usize {
            let start = 33 + i * 32;
            let mut comm = [0u8; 32];
            comm.copy_from_slice(&proof_bytes[start..start + 32]);
            bit_commitments.push(comm);
        }
        let proof = RangeProof {
            bit_commitments,
            bit_blindings: Vec::new(),
            bit_proofs: Vec::new(),
            response,
            bits,
        };
        self.proofs_verified += 1;
        if proof.verify() {
            self.proofs_valid += 1;
            ZkResult::Valid
        } else {
            self.proofs_invalid += 1;
            ZkResult::Invalid
        }
    }
}
