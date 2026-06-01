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

use super::read::read_symbol;
use super::state::SymbolLookup;
use crate::memory::addr::VirtAddr;
use alloc::string::String;

impl SymbolLookup {
    pub fn find_by_index(&self, index: usize) -> Option<(String, VirtAddr, u64)> {
        if index == 0 || index >= self.sym_count {
            return None;
        }
        let sym = unsafe { read_symbol(self.symtab, index)? };
        if sym.is_undefined() {
            return None;
        }
        let Ok(name_offset) = usize::try_from(sym.st_name) else {
            return None;
        };
        Some((self.name_or_empty(name_offset), self.base_addr + sym.st_value, sym.st_size))
    }
}
