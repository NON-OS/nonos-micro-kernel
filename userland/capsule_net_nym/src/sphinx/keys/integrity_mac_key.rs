// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

use super::super::constants::INTEGRITY_MAC_KEY_SIZE;
use super::offsets::INTEGRITY_MAC_KEY_AT;
use super::types::ExpandedSharedSecret;

impl ExpandedSharedSecret {
    pub fn integrity_mac_key(&self) -> [u8; INTEGRITY_MAC_KEY_SIZE] {
        let mut key = [0u8; INTEGRITY_MAC_KEY_SIZE];
        key.copy_from_slice(
            &self.0[INTEGRITY_MAC_KEY_AT..INTEGRITY_MAC_KEY_AT + INTEGRITY_MAC_KEY_SIZE],
        );
        key
    }
}
