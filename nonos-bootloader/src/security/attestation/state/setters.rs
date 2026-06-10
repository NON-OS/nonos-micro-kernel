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

impl AttestationState {
    pub fn set_kernel_hash(&mut self, hash: [u8; 32]) {
        self.kernel_hash = hash;
        self.extend_pcr_hash(PcrIndex::Kernel, &hash);
    }

    pub fn set_bootloader_hash(&mut self, hash: [u8; 32]) {
        self.bootloader_hash = hash;
        self.extend_pcr_hash(PcrIndex::Bootloader, &hash);
    }

    pub fn set_zk_verified(&mut self, verified: bool, ph: [u8; 32], commit: [u8; 32]) {
        self.zk_verified = verified;
        self.program_hash = ph;
        self.capsule_commitment = commit;
        self.extend_pcr_hash(PcrIndex::ZkProof, &ph);
    }

    pub fn set_signature_verified(&mut self, verified: bool) {
        self.sig_verified = verified;
    }
}
