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

use super::types::{KeyType, Store, StoreError};

impl Store {
    pub fn eth_secret(&mut self, id: u32, caller_pid: u32) -> Result<[u8; 32], StoreError> {
        let entry = self.entries.get_mut(&id).ok_or(StoreError::NotFound)?;
        if entry.owner_pid != caller_pid {
            return Err(StoreError::AccessDenied);
        }
        if entry.key_type != KeyType::Secp256k1Eth {
            return Err(StoreError::InvalidArgument);
        }
        if entry.locked {
            return Err(StoreError::Locked);
        }
        if entry.data.len() != 32 {
            return Err(StoreError::InvalidArgument);
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&entry.data);
        entry.use_count = entry.use_count.saturating_add(1);
        Ok(out)
    }
}
