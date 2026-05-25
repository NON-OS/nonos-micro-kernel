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

use alloc::{string::String, vec::Vec};

use crate::memory::addr::VirtAddr;

#[derive(Debug, Clone)]
pub struct DynLinkInfo {
    pub needed_libraries: Vec<String>,
    pub symtab: Option<VirtAddr>,
    pub strtab: Option<VirtAddr>,
    pub strtab_size: usize,
    pub sym_count: usize,
    pub rela_table: Option<VirtAddr>,
    pub rela_size: usize,
    pub plt_relocations: Option<VirtAddr>,
    pub plt_rela_size: usize,
    pub init: Option<VirtAddr>,
    pub fini: Option<VirtAddr>,
    pub init_array: Option<(VirtAddr, usize)>,
    pub fini_array: Option<(VirtAddr, usize)>,
}

impl DynLinkInfo {
    pub fn new() -> Self {
        Self {
            needed_libraries: Vec::new(), symtab: None, strtab: None, strtab_size: 0, sym_count: 0, rela_table: None,
            rela_size: 0, plt_relocations: None, plt_rela_size: 0, init: None, fini: None, init_array: None, fini_array: None,
        }
    }
}

impl Default for DynLinkInfo {
    fn default() -> Self { Self::new() }
}
