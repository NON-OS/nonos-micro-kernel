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
use crate::elf::types::constants::{class, data, ident, machine, ELF_MAGIC};
use crate::elf::types::elf_type;
use core::mem;

#[test]
fn test_elf_header_size() {
    assert_eq!(mem::size_of::<ElfHeader>(), ElfHeader::SIZE);
}

#[test]
fn test_elf_magic() {
    let mut header = ElfHeader::default();
    assert!(!header.is_valid_magic());
    header.ident[0..4].copy_from_slice(&ELF_MAGIC);
    assert!(header.is_valid_magic());
}

#[test]
fn test_elf_header_helpers() {
    let mut header = ElfHeader::default();
    header.ident[ident::EI_CLASS] = class::ELFCLASS64;
    header.ident[ident::EI_DATA] = data::ELFDATA2LSB;
    header.e_type = elf_type::ET_DYN;
    header.e_machine = machine::EM_X86_64;
    assert!(header.is_64bit());
    assert!(header.is_little_endian());
    assert!(header.is_executable());
    assert!(header.is_pie());
    assert!(header.is_x86_64());
    assert_eq!(header.type_name(), "DYN");
    assert_eq!(header.machine_name(), "AMD x86-64");
}

#[test]
fn test_elf_header_layout_helpers() {
    let mut header = ElfHeader::default();
    header.ident[ident::EI_VERSION] = 1;
    header.e_version = 1;
    assert!(header.version_is_current());
    assert!(header.has_native_header_size());
    assert!(header.has_native_program_header_size());
    assert!(header.has_native_section_header_size());
    assert!(header.has_valid_section_name_table_index());
}

#[test]
fn test_elf_header_rejects_bad_section_name_index() {
    let mut header = ElfHeader::default();
    header.e_shnum = 2;
    header.e_shstrndx = 3;
    assert!(!header.has_valid_section_name_table_index());
}
