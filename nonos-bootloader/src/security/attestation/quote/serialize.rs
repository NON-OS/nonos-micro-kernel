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

use super::types::AttestationQuote;
use alloc::vec::Vec;

impl AttestationQuote {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(512);
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&self.timestamp.to_le_bytes());
        buf.push(self.pcr_count as u8);
        for i in 0..self.pcr_count {
            buf.push(self.pcr_values[i].0);
            buf.extend_from_slice(&self.pcr_values[i].1);
        }
        buf.extend_from_slice(&self.kernel_hash);
        buf.extend_from_slice(&self.bootloader_hash);
        buf.push(self.zk_proof_verified as u8);
        buf.push(self.signature_verified as u8);
        buf.extend_from_slice(&self.program_hash);
        buf.extend_from_slice(&self.capsule_commitment);
        buf.extend_from_slice(&self.quote_signature);
        buf
    }
}
