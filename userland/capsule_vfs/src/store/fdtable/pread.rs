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

use alloc::vec::Vec;

use super::types::{Store, StoreError};

impl Store {
    pub fn pread(&mut self, fd: u32, owner_pid: u32, offset: u64, max: usize) -> Result<Vec<u8>, StoreError> {
        let file_idx = self.entry(fd, owner_pid)?.file_idx;
        let data = &self.files[file_idx].data;
        let off = (offset as usize).min(data.len());
        let avail = data.len() - off;
        let n = if max < avail { max } else { avail };
        Ok(data[off..off + n].to_vec())
    }
}
