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

use crate::elf::errors::{ElfError, ElfResult};
use crate::memory::addr::VirtAddr;

use super::header::SysvHashHeader;

pub struct SysvHashTable {
    pub(super) header: SysvHashHeader,
    pub(super) buckets: VirtAddr,
    pub(super) chains: VirtAddr,
    pub(super) symtab: VirtAddr,
    pub(super) strtab: VirtAddr,
    pub(super) strtab_size: usize,
}

impl SysvHashTable {
    pub fn new(hash_addr: VirtAddr, symtab: VirtAddr, strtab: VirtAddr, strtab_size: usize) -> ElfResult<Self> {
        let header = unsafe { ptr::read(hash_addr.as_u64() as *const SysvHashHeader) };
        if header.nbuckets == 0 {
            return Err(ElfError::InvalidHash);
        }
        let buckets = VirtAddr::new(hash_addr.as_u64() + 8);
        let chains = VirtAddr::new(buckets.as_u64() + (header.nbuckets as u64 * 4));
        Ok(Self { header, buckets, chains, symtab, strtab, strtab_size })
    }
}
