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

use crate::tcp::Tcb;

use super::types::{Table, TableError, TABLE_CAP};
use crate::state::Entry;

impl Table {
    pub fn insert(&mut self, owner: u32, parent: u32, tcb: Tcb) -> Result<u32, TableError> {
        if self.entries.len() >= TABLE_CAP {
            return Err(TableError::Full);
        }
        let handle = self.next_handle;
        self.next_handle = self.next_handle.wrapping_add(1).max(1);
        self.entries.push(Entry::new(owner, handle, parent, tcb));
        Ok(handle)
    }

    pub fn remove(&mut self, owner: u32, handle: u32) -> Result<(), TableError> {
        let idx = self
            .entries
            .iter()
            .position(|e| e.owner_pid == owner && e.handle == handle)
            .ok_or(TableError::NotFound)?;
        self.entries.swap_remove(idx);
        Ok(())
    }

    pub fn remove_by_handle(&mut self, handle: u32) {
        self.entries.retain(|e| e.handle != handle);
    }

    pub fn is_idle(&self) -> bool {
        self.entries.is_empty() && self.timers.is_empty()
    }

    pub fn entries_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }
}
