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

use super::state::ElfHeader;
use crate::elf::types::{constants::{class, data, elf_type, ident, machine, ELF_MAGIC}, ProgramHeader, SectionHeader};

impl ElfHeader {
    pub fn is_valid_magic(&self) -> bool { self.ident[0..4] == ELF_MAGIC }
    pub fn is_64bit(&self) -> bool { self.ident[ident::EI_CLASS] == class::ELFCLASS64 }
    pub fn is_little_endian(&self) -> bool { self.ident[ident::EI_DATA] == data::ELFDATA2LSB }
    pub fn version_is_current(&self) -> bool { self.ident[ident::EI_VERSION] == 1 && self.e_version == 1 }
    pub fn is_executable(&self) -> bool { self.e_type == elf_type::ET_EXEC || self.e_type == elf_type::ET_DYN }
    pub fn is_pie(&self) -> bool { self.e_type == elf_type::ET_DYN }
    pub fn is_x86_64(&self) -> bool { self.e_machine == machine::EM_X86_64 }
    pub fn has_native_header_size(&self) -> bool { usize::from(self.e_ehsize) == Self::SIZE }
    pub fn has_native_program_header_size(&self) -> bool { self.e_phnum == 0 || usize::from(self.e_phentsize) == ProgramHeader::SIZE }
    pub fn has_native_section_header_size(&self) -> bool { self.e_shnum == 0 || usize::from(self.e_shentsize) == SectionHeader::SIZE }
    pub fn has_valid_section_name_table_index(&self) -> bool { (self.e_shnum == 0 && self.e_shstrndx == 0) || self.e_shstrndx == 0 || self.e_shstrndx < self.e_shnum }
}
