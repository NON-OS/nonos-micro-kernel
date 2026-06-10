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
use super::types::{KeyId, MAX_KEYS, PK_LEN};
use super::util::{constant_time_eq, derive_keyid};

impl KeyStore {
    pub fn add_key(&mut self, pubkey: &[u8; PK_LEN], version: u32) -> Result<KeyId, &'static str> {
        let id = derive_keyid(pubkey);
        if self.is_revoked(&id) {
            return Err("key revoked");
        }
        if version < self.minimum_version {
            return Err("version too old");
        }
        for i in 0..self.count {
            if constant_time_eq(&self.keys[i], pubkey) {
                if version > self.versions[i] {
                    self.versions[i] = version;
                }
                return Ok(id);
            }
        }
        if self.count >= MAX_KEYS {
            return Err("keystore full");
        }
        self.keys[self.count] = *pubkey;
        self.versions[self.count] = version;
        self.count += 1;
        Ok(id)
    }
}
