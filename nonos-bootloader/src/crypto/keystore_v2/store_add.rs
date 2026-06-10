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

use super::store_core::KeystoreV2;
use super::types_consts::MAX_TRUSTED_KEYS;
use super::types_trusted_key::TrustedKey;
use super::util::constant_time_eq;

impl KeystoreV2 {
    pub fn add_key(&mut self, key: TrustedKey) -> Result<(), &'static str> {
        if self.key_count >= MAX_TRUSTED_KEYS {
            return Err("keystore full");
        }
        if self.is_revoked(&key.key_id) {
            return Err("key is revoked");
        }
        for i in 0..self.key_count {
            if constant_time_eq(&self.keys[i].key_id, &key.key_id) {
                return Err("key already exists");
            }
        }
        self.keys[self.key_count] = key;
        self.key_count += 1;
        Ok(())
    }
}
