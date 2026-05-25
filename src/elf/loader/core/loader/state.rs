// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use crate::elf::aslr::AslrManager;
use crate::elf::loader::ElfImage;
use crate::memory::addr::VirtAddr;
use alloc::{collections::BTreeMap, string::String};

pub(super) const DEFAULT_STATIC_BASE: u64 = 0x400000;
pub(super) const DEFAULT_PIE_BASE: u64 = 0x400000;

pub struct ElfLoader {
    pub(super) aslr_manager: AslrManager,
    pub(super) loaded_libraries: BTreeMap<String, ElfImage>,
    pub(super) symbol_cache: BTreeMap<String, VirtAddr>,
}

impl ElfLoader {
    pub fn new() -> Self {
        Self {
            aslr_manager: AslrManager::new(),
            loaded_libraries: BTreeMap::new(),
            symbol_cache: BTreeMap::new(),
        }
    }

    pub fn library_count(&self) -> usize {
        self.loaded_libraries.len()
    }

    pub fn symbol_count(&self) -> usize {
        self.symbol_cache.len()
    }

    pub fn clear_symbol_cache(&mut self) {
        self.symbol_cache.clear();
    }
}
