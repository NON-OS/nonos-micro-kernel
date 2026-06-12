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

use super::types_consts::MAX_TRUSTED_KEYS;
use super::types_trusted_key::TrustedKey;

pub struct KeystoreV2 {
    pub(super) keys: [TrustedKey; MAX_TRUSTED_KEYS],
    pub(super) key_count: usize,
    pub(super) minimum_version: u32,
    pub(super) revocations: [[u8; 32]; 16],
    pub(super) revocation_count: usize,
    pub(super) require_cosign: bool,
}

impl KeystoreV2 {
    pub const fn new() -> Self {
        Self {
            keys: [TrustedKey::empty(); MAX_TRUSTED_KEYS],
            key_count: 0,
            minimum_version: 1,
            revocations: [[0u8; 32]; 16],
            revocation_count: 0,
            require_cosign: false,
        }
    }
}
