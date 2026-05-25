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

use super::super::parse_tls_section;
use crate::elf::errors::ElfError;
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;

#[test]
fn test_parse_tls_rejects_mem_smaller_than_template() {
    let ph = ProgramHeader { p_type: phdr_type::PT_TLS, p_flags: 0, p_offset: 0, p_vaddr: 0, p_paddr: 0, p_filesz: 64, p_memsz: 32, p_align: 16 };
    assert!(matches!(parse_tls_section(&ph, VirtAddr::new(0)), Err(ElfError::TlsSectionError)));
}

#[test]
fn test_parse_tls_rejects_invalid_alignment() {
    let ph = ProgramHeader { p_type: phdr_type::PT_TLS, p_flags: 0, p_offset: 0, p_vaddr: 0, p_paddr: 0, p_filesz: 32, p_memsz: 64, p_align: 24 };
    assert!(matches!(parse_tls_section(&ph, VirtAddr::new(0)), Err(ElfError::AlignmentError)));
}
