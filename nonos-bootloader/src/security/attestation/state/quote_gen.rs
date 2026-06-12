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

use super::types::AttestationState;
use crate::security::attestation::pcr::PcrIndex;
use crate::security::attestation::quote::AttestationQuote;

impl AttestationState {
    pub fn generate_quote(&self, nonce: [u8; 32], ts: u64) -> AttestationQuote {
        let mut q = AttestationQuote::new(nonce, ts);
        q.add_pcr(PcrIndex::SecureBootState as u8, self.pcrs[7].value);
        q.add_pcr(PcrIndex::Bootloader as u8, self.pcrs[8].value);
        q.add_pcr(PcrIndex::Kernel as u8, self.pcrs[9].value);
        q.add_pcr(PcrIndex::ZkProof as u8, self.pcrs[10].value);
        q.add_pcr(PcrIndex::BootAudit as u8, self.pcrs[11].value);
        q.kernel_hash = self.kernel_hash;
        q.bootloader_hash = self.bootloader_hash;
        q.zk_proof_verified = self.zk_verified;
        q.signature_verified = self.sig_verified;
        q.program_hash = self.program_hash;
        q.capsule_commitment = self.capsule_commitment;
        q
    }

    pub fn generate_signed_quote(
        &self,
        nonce: [u8; 32],
        ts: u64,
        key: &ed25519_dalek::SigningKey,
    ) -> AttestationQuote {
        use ed25519_dalek::Signer;
        let mut q = self.generate_quote(nonce, ts);
        q.quote_signature = key.sign(&q.compute_quote_hash()).to_bytes();
        q
    }
}
