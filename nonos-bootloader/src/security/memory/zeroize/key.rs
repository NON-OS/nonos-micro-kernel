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

use super::core::zeroize_32;

pub struct SecureKey {
    key: [u8; 32],
    active: bool,
}

impl SecureKey {
    pub const fn empty() -> Self {
        Self { key: [0u8; 32], active: false }
    }

    pub fn load(&mut self, key_bytes: &[u8; 32]) {
        self.key.copy_from_slice(key_bytes);
        self.active = true;
    }

    pub fn get(&self) -> Option<&[u8; 32]> {
        if self.active {
            Some(&self.key)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        zeroize_32(&mut self.key);
        self.active = false;
    }
}

impl Drop for SecureKey {
    fn drop(&mut self) {
        self.clear();
    }
}
