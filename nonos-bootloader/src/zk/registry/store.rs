// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

extern crate alloc;
use super::types_entry::DynamicCircuitEntry;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub struct DynamicCircuitStore {
    entries: BTreeMap<[u8; 32], DynamicCircuitEntry>,
}

impl DynamicCircuitStore {
    pub const fn new() -> Self {
        Self { entries: BTreeMap::new() }
    }
    pub fn register(&mut self, entry: DynamicCircuitEntry) -> Result<(), &'static str> {
        if entry.vk_bytes.is_empty() {
            return Err("circuit: empty VK");
        }
        if self.entries.contains_key(&entry.program_hash) {
            return Err("circuit: duplicate program hash");
        }
        self.entries.insert(entry.program_hash, entry);
        Ok(())
    }
    pub fn unregister(&mut self, program_hash: &[u8; 32]) -> bool {
        self.entries.remove(program_hash).is_some()
    }
    pub fn lookup(&self, program_hash: &[u8; 32]) -> Option<&DynamicCircuitEntry> {
        self.entries.get(program_hash)
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn list_hashes(&self) -> Vec<[u8; 32]> {
        self.entries.keys().cloned().collect()
    }
}
