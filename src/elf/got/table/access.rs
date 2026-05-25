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

use core::ptr;

use crate::elf::errors::{ElfError, ElfResult};
use crate::memory::addr::VirtAddr;

use super::{constants::GOT_ENTRY_SIZE, entry::GotEntry, state::GlobalOffsetTable};

impl GlobalOffsetTable {
    pub fn get_entry(&self, index: usize) -> Option<&GotEntry> { self.entries.get(index) }
    pub fn get_entry_mut(&mut self, index: usize) -> Option<&mut GotEntry> { self.entries.get_mut(index) }

    pub fn read_entry(&self, index: usize) -> ElfResult<u64> {
        if index >= self.entry_count { return Err(ElfError::InvalidIndex); }
        let address = self.base.as_u64() + (index * GOT_ENTRY_SIZE) as u64;
        unsafe { Ok(ptr::read(address as *const u64)) }
    }

    pub fn write_entry(&mut self, index: usize, value: u64) -> ElfResult<()> {
        if index >= self.entry_count { return Err(ElfError::InvalidIndex); }
        let address = self.base.as_u64() + (index * GOT_ENTRY_SIZE) as u64;
        unsafe { ptr::write(address as *mut u64, value); }
        if let Some(entry) = self.entries.get_mut(index) {
            entry.resolve(value);
        }
        Ok(())
    }

    pub fn resolve_symbol(&mut self, index: usize, target: VirtAddr) -> ElfResult<()> { self.write_entry(index, target.as_u64()) }
    pub fn set_dynamic(&mut self, dynamic_addr: VirtAddr) -> ElfResult<()> { self.write_entry(0, dynamic_addr.as_u64()) }
    pub fn set_link_map(&mut self, link_map_addr: VirtAddr) -> ElfResult<()> { self.write_entry(1, link_map_addr.as_u64()) }
    pub fn set_plt_resolver(&mut self, resolver_addr: VirtAddr) -> ElfResult<()> { self.write_entry(2, resolver_addr.as_u64()) }
}
