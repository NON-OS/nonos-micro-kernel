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

use crate::elf::types::Symbol;

use super::state::SysvHashTable;

impl SysvHashTable {
    pub(super) fn compare_symbol_name(&self, sym_idx: usize, name: &str) -> bool {
        let sym_ptr = (self.symtab.as_u64() + (sym_idx * Symbol::SIZE) as u64) as *const Symbol;
        let sym = unsafe { ptr::read(sym_ptr) };
        let Ok(name_offset) = usize::try_from(sym.st_name) else {
            return false;
        };
        if name_offset >= self.strtab_size {
            return false;
        }
        unsafe {
            let str_ptr = (self.strtab.as_u64() + sym.st_name as u64) as *const u8;
            let name_bytes = name.as_bytes();
            for (i, &expected) in name_bytes.iter().enumerate() {
                if *str_ptr.add(i) != expected {
                    return false;
                }
            }
            *str_ptr.add(name_bytes.len()) == 0
        }
    }
}
