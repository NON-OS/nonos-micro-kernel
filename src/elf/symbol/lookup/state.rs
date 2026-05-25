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

pub struct SymbolLookup {
    pub(super) symtab: VirtAddr,
    pub(super) strtab: VirtAddr,
    pub(super) strtab_size: usize,
    pub(super) sym_count: usize,
    pub(super) base_addr: VirtAddr,
}

impl SymbolLookup {
    pub fn new(symtab: VirtAddr, strtab: VirtAddr, strtab_size: usize, sym_count: usize, base_addr: VirtAddr) -> Self {
        Self { symtab, strtab, strtab_size, sym_count, base_addr }
    }
}
