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
    pub fn rename(&mut self, old: &str, new: &str) -> StoreResult<()> {
        let idx = self.find(old).ok_or(StoreError::NotFound)?;
        if self.find(new).is_some() {
            return Err(StoreError::Exists);
        }
        if self.files[idx].is_dir {
            let from = {
                let mut s = String::from(old);
                s.push('/');
                s
            };
            let to = {
                let mut s = String::from(new);
                s.push('/');
                s
            };
            for f in self.files.iter_mut() {
                if let Some(rest) = f.name.strip_prefix(from.as_str()) {
                    let mut renamed = to.clone();
                    renamed.push_str(rest);
                    f.name = renamed;
                }
            }
        }
        self.files[idx].name = String::from(new);
        Ok(())
    }
}
