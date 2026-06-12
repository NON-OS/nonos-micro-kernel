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

use super::super::load_parts::load;
use crate::elf::errors::ElfError;
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;

#[test]
fn test_duplicate_dynamic_segment_is_rejected() {
    let headers = [dynamic_phdr(), dynamic_phdr()];
    assert!(matches!(load(&[], &headers, VirtAddr::new(0), 1), Err(ElfError::DynamicSectionError)));
}

#[test]
fn test_duplicate_tls_segment_is_rejected() {
    let headers = [tls_phdr(), tls_phdr()];
    assert!(matches!(load(&[], &headers, VirtAddr::new(0), 1), Err(ElfError::TlsSectionError)));
}

fn dynamic_phdr() -> ProgramHeader {
    ProgramHeader {
        p_type: phdr_type::PT_DYNAMIC,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_align: 8,
    }
}

fn tls_phdr() -> ProgramHeader {
    ProgramHeader {
        p_type: phdr_type::PT_TLS,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 0,
        p_memsz: 0,
        p_align: 8,
    }
}
