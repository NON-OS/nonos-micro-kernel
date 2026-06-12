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
use super::util::constant_time_eq;
use crate::crypto::keys::KeyId;

impl KeystoreV2 {
    pub fn revoke_key(&mut self, key_id: &KeyId) -> bool {
        if self.revocation_count >= 16 {
            return false;
        }
        for i in 0..self.key_count {
            if constant_time_eq(&self.keys[i].key_id, key_id) {
                self.keys[i].active = false;
            }
        }
        self.revocations[self.revocation_count] = *key_id;
        self.revocation_count += 1;
        true
    }

    pub fn is_revoked(&self, key_id: &KeyId) -> bool {
        for i in 0..self.revocation_count {
            if constant_time_eq(&self.revocations[i], key_id) {
                return true;
            }
        }
        false
    }

    pub fn set_minimum_version(&mut self, version: u32) {
        if version > self.minimum_version {
            self.minimum_version = version;
        }
    }
}
