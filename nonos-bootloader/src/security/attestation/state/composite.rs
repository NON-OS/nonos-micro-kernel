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
use crate::security::attestation::pcr::DS_ATTESTATION;

impl AttestationState {
    pub fn compute_composite_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new_derive_key(DS_ATTESTATION);
        for pcr in &self.pcrs {
            if pcr.extended {
                hasher.update(&[pcr.index]);
                hasher.update(&pcr.value);
            }
        }
        *hasher.finalize().as_bytes()
    }
}
