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

use super::types::{RevocationEntry, MAX_KEYS, MAX_REVOKED};

pub struct KeyStore {
    pub keys: [[u8; 32]; MAX_KEYS],
    pub versions: [u32; MAX_KEYS],
    pub count: usize,
    pub revoked: [RevocationEntry; MAX_REVOKED],
    pub revoked_count: usize,
    pub minimum_version: u32,
}

impl KeyStore {
    pub const fn new() -> Self {
        Self {
            keys: [[0u8; 32]; MAX_KEYS],
            versions: [0u32; MAX_KEYS],
            count: 0,
            revoked: [RevocationEntry::empty(); MAX_REVOKED],
            revoked_count: 0,
            minimum_version: 1,
        }
    }
}
