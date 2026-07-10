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

use super::types::{SeekWhence, Store, StoreError, MAX_FILE_BYTES};

impl Store {
    // Move an open fd's cursor. Positions past end-of-file are allowed up
    // to the per-file cap, matching POSIX: a later write zero-fills the
    // gap (the write path already resizes), and a read there returns
    // nothing. Negative results are invalid.
    pub fn seek(
        &mut self,
        fd: u32,
        owner_pid: u32,
        whence: SeekWhence,
        offset: i64,
    ) -> Result<u64, StoreError> {
        let (file_idx, pos) = {
            let entry = self.entry(fd, owner_pid)?;
            (entry.file_idx, entry.pos)
        };
        let base: i64 = match whence {
            SeekWhence::Set => 0,
            SeekWhence::Cur => pos as i64,
            SeekWhence::End => self.files[file_idx].data.len() as i64,
        };
        let target = base.checked_add(offset).ok_or(StoreError::Inval)?;
        if target < 0 || target > MAX_FILE_BYTES as i64 {
            return Err(StoreError::Inval);
        }
        if let Some(entry) = self.fds[fd as usize].as_mut() {
            entry.pos = target as usize;
        }
        Ok(target as u64)
    }
}
