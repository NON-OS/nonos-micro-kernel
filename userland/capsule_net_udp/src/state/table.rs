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

use super::bind::BindEntry;

pub const MAX_BINDS: usize = 64;

pub struct BindTable {
    entries: Vec<BindEntry>,
}

impl BindTable {
    pub const fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn find_by_port_mut(&mut self, port: u16) -> Option<&mut BindEntry> {
        self.entries.iter_mut().find(|b| b.port == port)
    }

    pub fn find_owned_mut(&mut self, pid: u32, port: u16) -> Option<&mut BindEntry> {
        self.entries.iter_mut().find(|b| b.port == port && b.owner_pid == pid)
    }

    pub fn insert(&mut self, entry: BindEntry) -> Result<(), TableError> {
        if self.entries.iter().any(|b| b.port == entry.port) {
            return Err(TableError::InUse);
        }
        if self.entries.len() >= MAX_BINDS {
            return Err(TableError::Full);
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn remove(&mut self, pid: u32, port: u16) -> Result<(), TableError> {
        let idx = self
            .entries
            .iter()
            .position(|b| b.port == port && b.owner_pid == pid)
            .ok_or(TableError::NotFound)?;
        self.entries.swap_remove(idx);
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TableError {
    InUse,
    Full,
    NotFound,
}
