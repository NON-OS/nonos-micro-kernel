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

use super::store::KeyStore;
use super::types::{KeyStatus, PK_LEN};
use super::util::{constant_time_eq, derive_keyid};

impl KeyStore {
    pub fn validate_key(&self, pubkey: &[u8; PK_LEN], version: u32) -> KeyStatus {
        let key_id = derive_keyid(pubkey);
        if self.is_revoked(&key_id) {
            return KeyStatus::Revoked;
        }
        if version < self.minimum_version {
            return KeyStatus::VersionTooOld;
        }
        for i in 0..self.count {
            if constant_time_eq(&self.keys[i], pubkey) {
                return KeyStatus::Valid;
            }
        }
        KeyStatus::Unknown
    }
}
