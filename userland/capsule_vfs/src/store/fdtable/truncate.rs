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

use super::types::{Store, StoreError, StoreResult, MAX_FILE_BYTES};

impl Store {
    // Set a file's length: shrink drops the tail, grow zero-fills. Directories
    // and oversized requests are rejected. Refreshes the modified time.
    pub fn truncate(&mut self, path: &str, size: u64) -> StoreResult<()> {
        let idx = self.find(path).ok_or(StoreError::NotFound)?;
        if self.files[idx].is_dir {
            return Err(StoreError::IsDir);
        }
        let new_len = size as usize;
        if new_len > MAX_FILE_BYTES {
            return Err(StoreError::Full);
        }
        // Securely erase the tail being dropped before it is freed.
        let data = &mut self.files[idx].data;
        if new_len < data.len() {
            super::zeroize::zeroize(&mut data[new_len..]);
        }
        data.resize(new_len, 0);
        self.files[idx].mtime = super::time::now_ms();
        Ok(())
    }
}
