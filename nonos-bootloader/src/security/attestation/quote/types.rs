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

#[derive(Clone)]
pub struct AttestationQuote {
    pub pcr_values: [(u8, [u8; 32]); 12],
    pub pcr_count: usize,
    pub kernel_hash: [u8; 32],
    pub bootloader_hash: [u8; 32],
    pub zk_proof_verified: bool,
    pub signature_verified: bool,
    pub program_hash: [u8; 32],
    pub capsule_commitment: [u8; 32],
    pub nonce: [u8; 32],
    pub timestamp: u64,
    pub quote_signature: [u8; 64],
}

impl AttestationQuote {
    pub fn new(nonce: [u8; 32], timestamp: u64) -> Self {
        Self {
            pcr_values: [(0, [0u8; 32]); 12],
            pcr_count: 0,
            kernel_hash: [0u8; 32],
            bootloader_hash: [0u8; 32],
            zk_proof_verified: false,
            signature_verified: false,
            program_hash: [0u8; 32],
            capsule_commitment: [0u8; 32],
            nonce,
            timestamp,
            quote_signature: [0u8; 64],
        }
    }

    pub fn add_pcr(&mut self, index: u8, value: [u8; 32]) {
        if self.pcr_count < 12 {
            self.pcr_values[self.pcr_count] = (index, value);
            self.pcr_count += 1;
        }
    }

    pub fn has_valid_measurements(&self) -> bool {
        self.zk_proof_verified && self.signature_verified
    }
}
