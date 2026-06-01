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

use crate::elf::errors::ElfResult;
use crate::memory::addr::VirtAddr;

use super::super::{gnu::GnuHashTable, sysv::SysvHashTable};

pub enum HashTable {
    Gnu(GnuHashTable),
    Sysv(SysvHashTable),
}

impl HashTable {
    pub fn gnu(hash_addr: VirtAddr, symtab: VirtAddr, strtab: VirtAddr, strtab_size: usize) -> ElfResult<Self> {
        Ok(Self::Gnu(GnuHashTable::new(hash_addr, symtab, strtab, strtab_size)?))
    }

    pub fn sysv(hash_addr: VirtAddr, symtab: VirtAddr, strtab: VirtAddr, strtab_size: usize) -> ElfResult<Self> {
        Ok(Self::Sysv(SysvHashTable::new(hash_addr, symtab, strtab, strtab_size)?))
    }

    pub fn lookup(&self, name: &str) -> Option<usize> {
        match self { Self::Gnu(table) => table.lookup(name), Self::Sysv(table) => table.lookup(name) }
    }

    pub fn is_gnu(&self) -> bool { matches!(self, Self::Gnu(_)) }
    pub fn is_sysv(&self) -> bool { matches!(self, Self::Sysv(_)) }
}
