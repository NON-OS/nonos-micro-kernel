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

use alloc::collections::BTreeMap;
use alloc::string::String;

pub const MAX_HANDLES: usize = 1024;

struct Handle {
    path: String,
    owner_pid: u32,
}

pub struct HandleTable {
    next_id: u64,
    table: BTreeMap<u64, Handle>,
}

impl HandleTable {
    pub const fn new() -> Self {
        Self { next_id: 1, table: BTreeMap::new() }
    }

    pub fn insert(&mut self, path: String, owner_pid: u32) -> Option<u64> {
        if self.table.len() >= MAX_HANDLES {
            return None;
        }
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.table.insert(id, Handle { path, owner_pid });
        Some(id)
    }

    pub fn path_for(&self, id: u64, sender_pid: u32) -> Result<&str, HandleError> {
        let h = self.table.get(&id).ok_or(HandleError::NotFound)?;
        if sender_pid != 0 && h.owner_pid != sender_pid {
            return Err(HandleError::Denied);
        }
        Ok(h.path.as_str())
    }

    pub fn remove(&mut self, id: u64, sender_pid: u32) -> Result<(), HandleError> {
        match self.table.get(&id) {
            None => Err(HandleError::NotFound),
            Some(h) if sender_pid != 0 && h.owner_pid != sender_pid => Err(HandleError::Denied),
            Some(_) => {
                self.table.remove(&id);
                Ok(())
            }
        }
    }
}

pub enum HandleError {
    NotFound,
    Denied,
}
