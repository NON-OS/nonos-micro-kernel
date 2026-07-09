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

use alloc::string::String;
use alloc::vec::Vec;

use super::types::{File, OpenFd, Store, StoreError, StoreResult, MAX_FILES, MODE_WRITE};

impl Store {
    pub fn open(
        &mut self,
        path: &str,
        owner_pid: u32,
        create: bool,
        truncate: bool,
        append: bool,
        writable: bool,
    ) -> Result<u32, StoreError> {
        let file_idx = match self.find(path) {
            Some(i) => i,
            None => self.create_file(path, create)?,
        };
        if self.files[file_idx].is_dir {
            return Err(StoreError::IsDir);
        }
        // The handle is writable only if the caller was allowed to write and the
        // file's own permissions permit it. Truncating a read-only file is
        // rejected rather than silently clearing it.
        let mode_writable = self.files[file_idx].mode & MODE_WRITE != 0;
        let writable = writable && mode_writable;
        if truncate {
            if !writable {
                return Err(StoreError::AccessDenied);
            }
            super::zeroize::zeroize(&mut self.files[file_idx].data);
            self.files[file_idx].data.clear();
        }
        let fd_slot = self.fds.iter().position(|s| s.is_none()).ok_or(StoreError::Full)?;
        let pos = if append { self.files[file_idx].data.len() } else { 0 };
        self.fds[fd_slot] = Some(OpenFd { file_idx, owner_pid, pos, append, writable });
        Ok(fd_slot as u32)
    }

    fn create_file(&mut self, path: &str, create: bool) -> StoreResult<usize> {
        if !create {
            return Err(StoreError::NotFound);
        }
        if self.files.len() >= MAX_FILES {
            return Err(StoreError::Full);
        }
        self.files.push(File::new(String::from(path), Vec::new(), false));
        Ok(self.files.len() - 1)
    }
}
