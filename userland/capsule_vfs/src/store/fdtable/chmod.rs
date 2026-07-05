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

use super::types::{Store, StoreError, StoreResult};

impl Store {
    // Set a file's permission bits.
    pub fn chmod(&mut self, path: &str, mode: u16) -> StoreResult<()> {
        let idx = self.find(path).ok_or(StoreError::NotFound)?;
        self.files[idx].mode = mode & 0o777;
        Ok(())
    }
}
