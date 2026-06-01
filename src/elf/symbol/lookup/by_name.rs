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

impl SymbolLookup {
    pub fn find_by_name(&self, name: &str) -> Option<(VirtAddr, u64)> {
        for index in 1..self.sym_count {
            let sym = unsafe { read_symbol(self.symtab, index)? };
            let Ok(name_offset) = usize::try_from(sym.st_name) else {
                continue;
            };
            if sym.is_undefined() || name_offset >= self.strtab_size {
                continue;
            }
            if self.name_or_empty(name_offset) == name {
                return Some((self.base_addr + sym.st_value, sym.st_size));
            }
        }
        None
    }
}
