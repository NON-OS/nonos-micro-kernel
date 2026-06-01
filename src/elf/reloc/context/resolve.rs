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

use super::state::RelocationContext;
use super::string::read_null_terminated_string;
use crate::elf::types::SymbolEntry;
use crate::elf::types::{sym_bind, sym_type};
use crate::memory::addr::VirtAddr;

impl<'a> RelocationContext<'a> {
    pub fn resolve_symbol(&self, sym_index: u32, base_addr: VirtAddr) -> Option<u64> {
        if sym_index == 0 {
            return Some(0);
        }
        let symtab = self.symbol_table?;
        unsafe {
            let sym_ptr = symtab.as_u64().checked_add(u64::from(sym_index) * SymbolEntry::SIZE as u64)?
                as *const SymbolEntry;
            let sym = core::ptr::read(sym_ptr);
            if sym.is_undefined() {
                return self.resolve_undefined_symbol(sym);
            }
            if sym.sym_type() == sym_type::STT_NOTYPE || sym.st_shndx == 0xFFF1 {
                Some(sym.st_value)
            } else {
                base_addr.as_u64().checked_add(sym.st_value)
            }
        }
    }

    unsafe fn resolve_undefined_symbol(&self, sym: SymbolEntry) -> Option<u64> {
        if let Some(addr) = self.lookup_cached_symbol(sym) {
            return Some(addr);
        }
        if sym.binding() == sym_bind::STB_WEAK {
            return Some(0);
        }
        None
    }

    unsafe fn lookup_cached_symbol(&self, sym: SymbolEntry) -> Option<u64> {
        let strtab = self.string_table?;
        let name_offset = usize::try_from(sym.st_name).ok()?;
        if name_offset >= self.string_table_size {
            return None;
        }
        let name_ptr = (strtab.as_u64() + sym.st_name as u64) as *const u8;
        let name = read_null_terminated_string(name_ptr, 256);
        self.symbol_cache.get(&name).map(|addr| addr.as_u64())
    }
}
