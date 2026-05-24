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

use super::entry::{TrayEntry, MAX_TRAY_ITEMS};
use crate::protocol::TRAY_LABEL_MAX;

pub struct TrayTable {
    entries: [TrayEntry; MAX_TRAY_ITEMS],
}

impl TrayTable {
    pub const fn new() -> Self {
        Self {
            entries: [TrayEntry {
                owner_pid: 0,
                tray_id: 0,
                label_len: 0,
                label: [0; TRAY_LABEL_MAX],
                in_use: false,
            }; MAX_TRAY_ITEMS],
        }
    }
    pub fn insert(&mut self, entry: TrayEntry) -> Result<(), ()> {
        if self.find(entry.owner_pid, entry.tray_id).is_some() {
            return Err(());
        }
        for slot in self.entries.iter_mut() {
            if !slot.in_use {
                *slot = entry;
                return Ok(());
            }
        }
        Err(())
    }
    pub fn find(&self, owner_pid: u32, tray_id: u32) -> Option<&TrayEntry> {
        self.entries.iter().find(|e| e.in_use && e.owner_pid == owner_pid && e.tray_id == tray_id)
    }
    pub fn find_mut(&mut self, owner_pid: u32, tray_id: u32) -> Option<&mut TrayEntry> {
        self.entries.iter_mut().find(|e| e.in_use && e.owner_pid == owner_pid && e.tray_id == tray_id)
    }
    pub fn remove(&mut self, owner_pid: u32, tray_id: u32) -> bool {
        for slot in self.entries.iter_mut() {
            if slot.in_use && slot.owner_pid == owner_pid && slot.tray_id == tray_id {
                *slot = TrayEntry::default();
                return true;
            }
        }
        false
    }
}
