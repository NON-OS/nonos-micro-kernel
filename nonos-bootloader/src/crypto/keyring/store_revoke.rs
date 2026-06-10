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

use super::store_core::KeyStore;
use super::types::{KeyId, RevocationEntry, RevocationReason, MAX_REVOKED};
use super::util::constant_time_eq;

impl KeyStore {
    pub fn is_revoked(&self, key_id: &KeyId) -> bool {
        for i in 0..self.revoked_count {
            if constant_time_eq(&self.revoked[i].key_id, key_id) {
                return true;
            }
        }
        false
    }

    pub fn revoke_key(&mut self, key_id: KeyId, reason: RevocationReason, timestamp: u64) -> bool {
        if self.revoked_count >= MAX_REVOKED {
            return false;
        }
        if self.is_revoked(&key_id) {
            return true;
        }
        self.revoked[self.revoked_count] =
            RevocationEntry { key_id, revoked_at: timestamp, reason };
        self.revoked_count += 1;
        true
    }
}
