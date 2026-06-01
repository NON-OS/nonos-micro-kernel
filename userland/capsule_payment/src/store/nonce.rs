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

use super::types::State;

impl State {
    pub fn next_nonce(&mut self, owner_pid: u32, wallet_id: u32, publisher: &[u8; 20]) -> u64 {
        let mut key = [0u8; 40];
        key[0..4].copy_from_slice(&owner_pid.to_le_bytes());
        key[4..8].copy_from_slice(&wallet_id.to_le_bytes());
        key[8..28].copy_from_slice(publisher);
        let slot = self.nonces.entry(key).or_insert(0);
        *slot = slot.saturating_add(1);
        *slot
    }
}
