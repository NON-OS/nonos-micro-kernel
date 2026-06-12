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

use super::state::DynLinkInfo;

impl DynLinkInfo {
    pub fn has_relocations(&self) -> bool {
        self.rela_table.is_some() || self.plt_relocations.is_some()
    }
    pub fn has_symbols(&self) -> bool {
        self.symtab.is_some()
    }
    pub fn has_strings(&self) -> bool {
        self.strtab.is_some() && self.strtab_size > 0
    }
    pub fn has_init(&self) -> bool {
        self.init.is_some() || self.init_array.is_some()
    }
    pub fn has_fini(&self) -> bool {
        self.fini.is_some() || self.fini_array.is_some()
    }
    pub fn string_table_end(&self) -> Option<VirtAddr> {
        self.strtab.map(|addr| addr + self.strtab_size as u64)
    }

    pub fn is_empty(&self) -> bool {
        self.needed_libraries.is_empty()
            && self.symtab.is_none()
            && self.rela_table.is_none()
            && self.plt_relocations.is_none()
            && self.init.is_none()
            && self.fini.is_none()
            && self.init_array.is_none()
            && self.fini_array.is_none()
    }
}
