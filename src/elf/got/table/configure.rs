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

use crate::memory::addr::VirtAddr;

use super::{constants::PLT_ENTRY_SIZE, state::GlobalOffsetTable};

impl GlobalOffsetTable {
    pub fn with_plt(mut self, plt_base: VirtAddr, plt_size: usize) -> Self {
        self.plt_base = Some(plt_base);
        self.plt_entry_count = plt_size / PLT_ENTRY_SIZE;
        self
    }

    pub fn base(&self) -> VirtAddr {
        self.base
    }
    pub fn entry_count(&self) -> usize {
        self.entry_count
    }
    pub fn plt_entry_count(&self) -> usize {
        self.plt_entry_count
    }
    pub fn unresolved_count(&self) -> usize {
        self.entries.iter().filter(|entry| !entry.resolved).count()
    }
    pub fn iter(&self) -> impl Iterator<Item = &super::entry::GotEntry> {
        self.entries.iter()
    }
}
