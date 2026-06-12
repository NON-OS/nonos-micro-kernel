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

use super::constants::DS_ATTESTATION;

#[derive(Clone, Copy)]
pub struct PcrValue {
    pub index: u8,
    pub value: [u8; 32],
    pub extended: bool,
}

impl PcrValue {
    pub const fn empty(index: u8) -> Self {
        Self { index, value: [0u8; 32], extended: false }
    }

    pub fn extend(&mut self, data: &[u8]) {
        let mut hasher = blake3::Hasher::new_derive_key(DS_ATTESTATION);
        hasher.update(&self.value);
        hasher.update(data);
        self.value = *hasher.finalize().as_bytes();
        self.extended = true;
    }

    pub fn extend_hash(&mut self, hash: &[u8; 32]) {
        let mut hasher = blake3::Hasher::new_derive_key(DS_ATTESTATION);
        hasher.update(&self.value);
        hasher.update(hash);
        self.value = *hasher.finalize().as_bytes();
        self.extended = true;
    }
}
