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
use super::types_key::KeyType;
use super::types_result::KeyValidationResult;
use super::types_trusted_key::TrustedKey;
use super::util::constant_time_eq;
use crate::crypto::keys::{KeyId, PK_LEN};

impl KeystoreV2 {
    pub fn find_key(&self, key_id: &KeyId) -> Option<&TrustedKey> {
        for i in 0..self.key_count {
            if constant_time_eq(&self.keys[i].key_id, key_id) {
                return Some(&self.keys[i]);
            }
        }
        None
    }

    pub fn find_key_by_pubkey(&self, pubkey: &[u8; PK_LEN]) -> Option<&TrustedKey> {
        for i in 0..self.key_count {
            if constant_time_eq(&self.keys[i].public_key, pubkey) {
                return Some(&self.keys[i]);
            }
        }
        None
    }

    pub fn get_active_keys(&self, timestamp: u64) -> impl Iterator<Item = &TrustedKey> {
        self.keys[..self.key_count].iter().filter(move |k| {
            k.is_valid_at(timestamp, self.minimum_version) == KeyValidationResult::Valid
        })
    }

    pub fn get_primary_key(&self, timestamp: u64) -> Option<&TrustedKey> {
        self.get_active_keys(timestamp).find(|k| k.key_type == KeyType::Primary)
    }
}
