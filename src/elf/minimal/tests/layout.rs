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

use super::super::validate_elf_detailed;
use super::support::make_valid_elf_header;
use crate::elf::errors::ElfError;
use crate::elf::types::{ElfHeader, ProgramHeader, SectionHeader};

#[test]
fn test_validate_elf_rejects_invalid_header_size() {
    let mut header = make_valid_elf_header();
    header[52..54].copy_from_slice(&(ElfHeader::SIZE as u16 - 1).to_le_bytes());
    assert!(matches!(validate_elf_detailed(&header), Err(ElfError::InvalidHeaderSize)));
}

#[test]
fn test_validate_elf_rejects_invalid_program_header_size() {
    let mut header = make_valid_elf_header();
    header[54..56].copy_from_slice(&(ProgramHeader::SIZE as u16 - 1).to_le_bytes());
    assert!(matches!(validate_elf_detailed(&header), Err(ElfError::InvalidProgramHeaderSize)));
}

#[test]
fn test_validate_elf_rejects_invalid_section_header_size() {
    let mut header = make_valid_elf_header();
    header[58..60].copy_from_slice(&1u16.to_le_bytes());
    header[60..62].copy_from_slice(&1u16.to_le_bytes());
    assert!(matches!(validate_elf_detailed(&header), Err(ElfError::InvalidSectionHeaderSize)));
}

#[test]
fn test_validate_elf_rejects_invalid_section_name_index() {
    let mut header = make_valid_elf_header();
    header[58..60].copy_from_slice(&(SectionHeader::SIZE as u16).to_le_bytes());
    header[60..62].copy_from_slice(&2u16.to_le_bytes());
    header[62..64].copy_from_slice(&3u16.to_le_bytes());
    assert!(matches!(validate_elf_detailed(&header), Err(ElfError::InvalidIndex)));
}
