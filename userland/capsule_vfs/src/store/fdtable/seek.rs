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
    pub fn seek(&mut self, fd: u32, owner_pid: u32, whence: u16, offset: i64) -> StoreResult<u64> {
        let (file_idx, pos) = {
            let e = self.entry(fd, owner_pid)?;
            (e.file_idx, e.pos)
        };
        let base: i64 = match whence {
            0 => 0,
            1 => pos as i64,
            2 => self.files[file_idx].data.len() as i64,
            _ => return Err(StoreError::Invalid),
        };
        let new = base.checked_add(offset).ok_or(StoreError::Invalid)?;
        if new < 0 {
            return Err(StoreError::Invalid);
        }
        if let Some(e) = self.fds[fd as usize].as_mut() {
            e.pos = new as usize;
        }
        Ok(new as u64)
    }
}
