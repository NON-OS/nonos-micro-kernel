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

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::types::{File, Store, StoreError, StoreResult, MAX_FILES};

impl Store {
    // Copy `src` to `dst` inside the store, no size-capped IPC round-trip. A
    // file is duplicated with its data; a directory with `recursive` clones the
    // whole subtree, rewriting each descendant's `src/` prefix to `dst/`. The
    // destination must not already exist. Returns the number of entries added.
    pub fn copy(&mut self, src: &str, dst: &str, recursive: bool) -> StoreResult<u32> {
        let src_idx = self.find(src).ok_or(StoreError::NotFound)?;
        if self.find(dst).is_some() {
            return Err(StoreError::Exists);
        }
        if !self.files[src_idx].is_dir {
            if self.files.len() >= MAX_FILES {
                return Err(StoreError::Full);
            }
            let data = self.files[src_idx].data.clone();
            self.files.push(File::new(String::from(dst), data, false));
            return Ok(1);
        }
        let mut additions: Vec<File> = Vec::new();
        additions.push(File::new(String::from(dst), Vec::new(), true));
        if recursive {
            let mut src_prefix = String::from(src);
            src_prefix.push('/');
            for f in &self.files {
                let Some(suffix) = f.name.strip_prefix(src_prefix.as_str()) else { continue };
                let new_name = format!("{dst}/{suffix}");
                if self.find(&new_name).is_some() {
                    return Err(StoreError::Exists);
                }
                additions.push(File::new(new_name, f.data.clone(), f.is_dir));
            }
        }
        if self.files.len() + additions.len() > MAX_FILES {
            return Err(StoreError::Full);
        }
        let count = additions.len() as u32;
        self.files.append(&mut additions);
        Ok(count)
    }
}
