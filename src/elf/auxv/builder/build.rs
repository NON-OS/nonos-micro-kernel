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

extern crate alloc;

use super::state::{AuxvBuilder, CLOCK_TICKS_PER_SEC, PAGE_SIZE};
use crate::elf::loader::ElfImage;
use crate::elf::types::ProgramHeader;
use crate::memory::addr::VirtAddr;
use alloc::vec::Vec;

impl AuxvBuilder {
    pub fn from_elf_image(image: &ElfImage, phdr_addr: VirtAddr, phnum: u16) -> Self {
        let mut builder = Self::new();
        builder
            .set_phdr(phdr_addr)
            .set_phent(ProgramHeader::SIZE as u64)
            .set_phnum(phnum as u64)
            .set_pagesz(PAGE_SIZE)
            .set_base(image.base_addr)
            .set_entry(image.entry_point)
            .set_flags(0)
            .set_uid(0)
            .set_euid(0)
            .set_gid(0)
            .set_egid(0)
            .set_clktck(CLOCK_TICKS_PER_SEC)
            .set_secure(false);
        builder
    }

    pub fn build(mut self) -> Vec<super::super::AuxEntry> {
        self.entries.push(super::super::AuxEntry::null());
        self.entries
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn size_bytes(&self) -> usize {
        (self.entries.len() + 1) * super::super::AuxEntry::SIZE
    }
}

impl Default for AuxvBuilder {
    fn default() -> Self {
        Self::new()
    }
}
