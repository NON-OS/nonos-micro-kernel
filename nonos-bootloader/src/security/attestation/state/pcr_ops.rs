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
use crate::security::attestation::pcr::{PcrIndex, PcrValue, MAX_PCRS};

impl AttestationState {
    pub fn extend_pcr(&mut self, index: PcrIndex, data: &[u8]) {
        let idx = index as usize;
        if idx < MAX_PCRS {
            self.pcrs[idx].extend(data);
        }
    }

    pub fn extend_pcr_hash(&mut self, index: PcrIndex, hash: &[u8; 32]) {
        let idx = index as usize;
        if idx < MAX_PCRS {
            self.pcrs[idx].extend_hash(hash);
        }
    }

    pub fn get_pcr(&self, index: PcrIndex) -> &PcrValue {
        &self.pcrs[index as usize]
    }
}
