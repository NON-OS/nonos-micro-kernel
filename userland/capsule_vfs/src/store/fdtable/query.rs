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
    pub fn stat(&self, path: &str) -> Result<(u64, bool), StoreError> {
        match self.find(path) {
            Some(i) => Ok((self.files[i].data.len() as u64, self.files[i].is_dir)),
            None => Err(StoreError::NotFound),
        }
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
