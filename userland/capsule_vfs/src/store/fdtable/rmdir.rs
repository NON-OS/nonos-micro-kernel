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

use super::types::{Store, StoreError, StoreResult};

impl Store {
    // Remove a directory. Without `recursive` an empty directory is removed and
    // a non-empty one returns NotEmpty; with `recursive` the directory and its
    // whole subtree go at once. Open fds pointing at removed files are closed,
    // and the survivors are reindexed to track the compacted file list.
    pub fn rmdir(&mut self, path: &str, recursive: bool) -> StoreResult<()> {
        self.find(path).ok_or(StoreError::NotFound)?;
        let mut prefix = String::from(path);
        prefix.push('/');
        let has_children = self.files.iter().any(|f| f.name.starts_with(prefix.as_str()));
        if has_children && !recursive {
            return Err(StoreError::NotEmpty);
        }
        let remove: Vec<bool> = self
            .files
            .iter()
            .map(|f| f.name == path || f.name.starts_with(prefix.as_str()))
            .collect();
        for slot in self.fds.iter_mut() {
            if let Some(fd) = slot.as_mut() {
                if remove[fd.file_idx] {
                    *slot = None;
                } else {
                    fd.file_idx -= remove[..fd.file_idx].iter().filter(|&&r| r).count();
                }
            }
        }
        for (i, file) in self.files.iter_mut().enumerate() {
            if remove[i] {
                super::zeroize::zeroize(&mut file.data);
            }
        }
        let mut i = 0usize;
        self.files.retain(|_| {
            let keep = !remove[i];
            i += 1;
            keep
        });
        Ok(())
    }
}
