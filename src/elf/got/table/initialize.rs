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

use crate::elf::errors::ElfResult;
use crate::memory::addr::VirtAddr;

use super::{
    constants::GOT_ENTRY_SIZE,
    entry::{GotEntry, GotEntryType},
    state::GlobalOffsetTable,
};

impl GlobalOffsetTable {
    pub fn initialize(&mut self) -> ElfResult<()> {
        self.entries.clear();
        for index in 0..self.entry_count {
            let address = VirtAddr::new(self.base.as_u64() + (index * GOT_ENTRY_SIZE) as u64);
            let value = unsafe { ptr::read(address.as_u64() as *const u64) };
            let entry_type = match index {
                0 => GotEntryType::Dynamic,
                1 => GotEntryType::LinkMap,
                2 => GotEntryType::PltResolver,
                _ => GotEntryType::Symbol(index - 3),
            };
            self.entries.push(GotEntry::new(index, address, value, entry_type));
        }
        Ok(())
    }
}
