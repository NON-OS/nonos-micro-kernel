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
    pub fn find_containing(&self, addr: VirtAddr) -> Option<(String, VirtAddr, u64)> {
        let target = addr.as_u64();
        let mut best_match = None;
        let mut best_distance = u64::MAX;
        for index in 1..self.sym_count {
            let sym = unsafe { read_symbol(self.symtab, index)? };
            if sym.is_undefined() {
                continue;
            }
            let sym_addr = self.base_addr.as_u64() + sym.st_value;
            let sym_end = sym_addr + sym.st_size;
            if target >= sym_addr && target < sym_end && target - sym_addr < best_distance {
                let Ok(name_offset) = usize::try_from(sym.st_name) else {
                    continue;
                };
                best_distance = target - sym_addr;
                best_match = Some((self.name_or_empty(name_offset), VirtAddr::new(sym_addr), sym.st_size));
            }
        }
        best_match
    }
}
