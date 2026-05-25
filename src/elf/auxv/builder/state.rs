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

extern crate alloc;

use super::super::types::AuxEntry;
use alloc::vec::Vec;

pub const MAX_AUXV_ENTRIES: usize = 32;
pub const PAGE_SIZE: u64 = 4096;
pub const CLOCK_TICKS_PER_SEC: u64 = 100;

pub struct AuxvBuilder {
    pub(super) entries: Vec<AuxEntry>,
}

impl AuxvBuilder {
    pub fn new() -> Self {
        Self { entries: Vec::with_capacity(MAX_AUXV_ENTRIES) }
    }

    pub fn add(&mut self, a_type: u64, a_val: u64) -> &mut Self {
        self.entries.push(AuxEntry::new(a_type, a_val));
        self
    }

    pub fn add_entry(&mut self, entry: AuxEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }
}
