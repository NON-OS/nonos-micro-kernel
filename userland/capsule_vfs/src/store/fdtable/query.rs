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
    pub fn stat(&self, path: &str) -> Result<(u64, bool, u64, u16), StoreError> {
        match self.find(path) {
            Some(i) => {
                let f = &self.files[i];
                // A directory reports its immediate child count in place of a
                // byte size, so a listing can show how full each folder is.
                let size = if f.is_dir { self.child_count(path) } else { f.data.len() as u64 };
                Ok((size, f.is_dir, f.mtime, f.mode))
            }
            None => Err(StoreError::NotFound),
        }
    }

    // Number of entries directly under `path` (one level deep).
    fn child_count(&self, path: &str) -> u64 {
        let mut prefix = alloc::string::String::from(path);
        prefix.push('/');
        self.files
            .iter()
            .filter(|f| {
                f.name.strip_prefix(prefix.as_str()).is_some_and(|rest| !rest.contains('/'))
            })
            .count() as u64
    }

    pub fn list(&self, prefix: &str, max_bytes: usize) -> Vec<u8> {
        let mut out = Vec::new();
        for f in self.files.iter() {
            if !f.name.starts_with(prefix) {
                continue;
            }
            let mut nb = Vec::from(f.name.as_bytes());
            if f.is_dir {
                nb.push(b'/');
            }
            if nb.len() > 255 || out.len() + 1 + nb.len() > max_bytes {
                continue;
            }
            out.push(nb.len() as u8);
            out.extend_from_slice(&nb);
        }
        out
    }
}
