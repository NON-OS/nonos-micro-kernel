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
use crate::elf::types::{sym_bind, sym_type, Symbol};
use crate::memory::addr::VirtAddr;

use super::{resolved::ResolvedSymbol, state::SymbolResolver, string::read_symbol_name};

impl SymbolResolver {
    pub fn parse_symbols(
        &mut self, symtab: VirtAddr, strtab: VirtAddr, strtab_size: usize, sym_count: usize, base_addr: VirtAddr,
        library_id: usize,
    ) -> ElfResult<usize> {
        let mut registered = 0;
        for i in 1..sym_count {
            unsafe {
                let sym_ptr = (symtab.as_u64() + (i * Symbol::SIZE) as u64) as *const Symbol;
                let sym = ptr::read(sym_ptr);
                let binding = sym.binding();
                let sym_kind = sym.sym_type();
                if sym.is_undefined() || (binding != sym_bind::STB_GLOBAL && binding != sym_bind::STB_WEAK) {
                    continue;
                }
                let Ok(name_offset) = usize::try_from(sym.st_name) else {
                    continue;
                };
                if name_offset >= strtab_size {
                    continue;
                }
                let name_ptr = (strtab.as_u64() + sym.st_name as u64) as *const u8;
                let name = read_symbol_name(name_ptr, strtab_size - name_offset);
                if name.is_empty() {
                    continue;
                }
                let address = if sym_kind == sym_type::STT_TLS { VirtAddr::new(sym.st_value) } else { base_addr + sym.st_value };
                self.register_symbol(ResolvedSymbol { name, address, size: sym.st_size, binding, sym_type: sym_kind, library_id });
                registered += 1;
            }
        }
        self.add_library(library_id);
        Ok(registered)
    }
}
