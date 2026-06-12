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

use super::super::super::plan::build;
use crate::elf::errors::ElfError;
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;

#[test]
fn test_build_rejects_invalid_alignment() {
    let ph = ProgramHeader {
        p_type: phdr_type::PT_LOAD,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_align: 24,
    };
    assert!(matches!(build(&[], &ph, VirtAddr::new(0)), Err(ElfError::AlignmentError)));
}

#[test]
fn test_build_rejects_vaddr_offset_alignment_mismatch() {
    let ph = ProgramHeader {
        p_type: phdr_type::PT_LOAD,
        p_flags: 0,
        p_offset: 0x1000,
        p_vaddr: 0x1800,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_align: 0x1000,
    };
    assert!(matches!(build(&[], &ph, VirtAddr::new(0)), Err(ElfError::AlignmentError)));
}
