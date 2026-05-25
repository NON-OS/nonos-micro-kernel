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

use crate::elf::types::RelaEntry;
use crate::memory::addr::VirtAddr;
use alloc::{string::String, vec::Vec};
use core::mem;

#[derive(Debug, Clone)]
pub struct DynamicInfo {
    pub needed_libraries: Vec<String>,
    pub symbol_table: Option<VirtAddr>,
    pub string_table: Option<VirtAddr>,
    pub string_table_size: usize,
    pub rela_table: Option<VirtAddr>,
    pub rela_size: usize,
    pub plt_relocations: Option<VirtAddr>,
    pub plt_rela_size: usize,
    pub init_function: Option<VirtAddr>,
    pub fini_function: Option<VirtAddr>,
}

impl DynamicInfo {
    pub fn new() -> Self {
        Self {
            needed_libraries: Vec::new(),
            symbol_table: None,
            string_table: None,
            string_table_size: 0,
            rela_table: None,
            rela_size: 0,
            plt_relocations: None,
            plt_rela_size: 0,
            init_function: None,
            fini_function: None,
        }
    }

    pub fn needs_relocation(&self) -> bool {
        self.rela_table.is_some() || self.plt_relocations.is_some()
    }

    pub fn needs_linking(&self) -> bool {
        !self.needed_libraries.is_empty()
    }

    pub fn rela_count(&self) -> usize {
        self.rela_size / mem::size_of::<RelaEntry>()
    }

    pub fn plt_rela_count(&self) -> usize {
        self.plt_rela_size / mem::size_of::<RelaEntry>()
    }
}

impl Default for DynamicInfo {
    fn default() -> Self {
        Self::new()
    }
}
