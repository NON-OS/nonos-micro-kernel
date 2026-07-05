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

use super::types::{File, Store, StoreError, StoreResult, MAX_FILES};

impl Store {
    // Create `path` and any missing ancestor directories (mkdir -p), so a
    // nested path never leaves an orphaned prefix in the flat namespace.
    pub fn mkdir(&mut self, path: &str) -> StoreResult<()> {
        if self.find(path).is_some() {
            return Err(StoreError::Exists);
        }
        let mut acc = String::new();
        for comp in path.trim_start_matches('/').split('/') {
            if comp.is_empty() {
                continue;
            }
            acc.push('/');
            acc.push_str(comp);
            if self.find(&acc).is_none() {
                if self.files.len() >= MAX_FILES {
                    return Err(StoreError::Full);
                }
                self.files.push(File::new(acc.clone(), Vec::new(), true));
            }
        }
        Ok(())
    }
}
