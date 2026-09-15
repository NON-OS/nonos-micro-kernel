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

//! What a capsule may hand to the block store: its own files.

use alloc::vec::Vec;

use super::types::{Store, StoreError, StoreResult};

impl Store {
    /// The bytes of `path` for persisting, when `pid` created the file. The
    /// pid is the one the kernel stamped on the request, so this is the
    /// attested sender and not a name or a path prefix standing in for one.
    pub fn persistable(&self, path: &str, pid: u32) -> StoreResult<Vec<u8>> {
        let file = &self.files[self.find(path).ok_or(StoreError::NotFound)?];
        if file.owner != pid {
            return Err(StoreError::AccessDenied);
        }
        Ok(file.data.clone())
    }
}
