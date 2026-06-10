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

use super::types::{Store, StoreError, StoreResult};

impl Store {
    pub fn unlink(&mut self, path: &str) -> StoreResult<()> {
        let idx = self.find(path).ok_or(StoreError::NotFound)?;
        if self.files[idx].is_dir {
            let mut prefix = String::from(path);
            prefix.push('/');
            if self.files.iter().any(|f| f.name.starts_with(prefix.as_str())) {
                return Err(StoreError::NotEmpty);
            }
        }
        for slot in self.fds.iter_mut() {
            match slot {
                Some(fd) if fd.file_idx == idx => *slot = None,
                Some(fd) if fd.file_idx > idx => fd.file_idx -= 1,
                _ => {}
            }
        }
        self.files.remove(idx);
        Ok(())
    }
}
