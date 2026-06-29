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

use super::types::{Store, StoreError, MAX_FILE_BYTES};

impl Store {
    pub fn pwrite(&mut self, fd: u32, owner_pid: u32, offset: u64, bytes: &[u8]) -> Result<u32, StoreError> {
        let (file_idx, writable) = {
            let e = self.entry(fd, owner_pid)?;
            (e.file_idx, e.writable)
        };
        if !writable {
            return Err(StoreError::AccessDenied);
        }
        let start = offset as usize;
        let end = start.saturating_add(bytes.len());
        if end > MAX_FILE_BYTES {
            return Err(StoreError::Full);
        }
        let data = &mut self.files[file_idx].data;
        if end > data.len() {
            data.resize(end, 0);
        }
        data[start..end].copy_from_slice(bytes);
        Ok(bytes.len() as u32)
    }
}
