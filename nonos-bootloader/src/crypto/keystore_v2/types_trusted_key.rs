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

use super::types_key::KeyType;
use super::types_result::KeyValidationResult;
use crate::crypto::keys::{derive_keyid, KeyId, PK_LEN};

#[derive(Clone, Copy)]
pub struct TrustedKey {
    pub key_id: KeyId,
    pub public_key: [u8; PK_LEN],
    pub version: u32,
    pub added_at: u64,
    pub valid_from: u64,
    pub valid_until: u64,
    pub key_type: KeyType,
    pub active: bool,
}

impl TrustedKey {
    pub const fn empty() -> Self {
        Self {
            key_id: [0u8; 32],
            public_key: [0u8; 32],
            version: 0,
            added_at: 0,
            valid_from: 0,
            valid_until: 0,
            key_type: KeyType::Primary,
            active: false,
        }
    }
    pub fn new(
        public_key: [u8; PK_LEN],
        version: u32,
        valid_from: u64,
        valid_until: u64,
        key_type: KeyType,
    ) -> Self {
        Self {
            key_id: derive_keyid(&public_key),
            public_key,
            version,
            added_at: 0,
            valid_from,
            valid_until,
            key_type,
            active: true,
        }
    }
    pub fn is_valid_at(&self, timestamp: u64, min_ver: u32) -> KeyValidationResult {
        if !self.active {
            return KeyValidationResult::Revoked;
        }
        if timestamp < self.valid_from {
            return KeyValidationResult::NotYetValid;
        }
        if self.valid_until > 0 && timestamp > self.valid_until {
            return KeyValidationResult::Expired;
        }
        if self.version < min_ver {
            return KeyValidationResult::VersionTooOld;
        }
        KeyValidationResult::Valid
    }

    pub fn zeroize(&mut self) {
        crate::security::memory::zeroize_32(&mut self.key_id);
        crate::security::memory::zeroize_32(&mut self.public_key);
        self.version = 0;
        self.added_at = 0;
        self.valid_from = 0;
        self.valid_until = 0;
        self.active = false;
    }
}
