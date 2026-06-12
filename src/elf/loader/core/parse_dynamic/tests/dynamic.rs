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

use super::super::parse_dynamic_section;
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;
use alloc::vec::Vec;

#[test]
fn test_parse_dynamic_maps_strtab_vaddr_to_file_offset() {
    let mut elf = Vec::new();
    push(&mut elf, 5, 0x2010);
    push(&mut elf, 10, 11);
    push(&mut elf, 1, 0);
    push(&mut elf, 0, 0);
    elf.resize(0x40, 0);
    elf.extend_from_slice(b"libdemo.so\0");
    let dynamic = ProgramHeader {
        p_type: phdr_type::PT_DYNAMIC,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0x3000,
        p_paddr: 0,
        p_filesz: 64,
        p_memsz: 64,
        p_align: 8,
    };
    let load = ProgramHeader {
        p_type: phdr_type::PT_LOAD,
        p_flags: 0,
        p_offset: 0x30,
        p_vaddr: 0x2000,
        p_paddr: 0,
        p_filesz: 0x20,
        p_memsz: 0x20,
        p_align: 0x1000,
    };
    let info = parse_dynamic_section(&elf, &dynamic, &[load], VirtAddr::new(0x400000)).unwrap();
    assert_eq!(info.needed_libraries, ["libdemo.so"]);
}

#[test]
fn test_parse_dynamic_rejects_misaligned_entry_size() {
    let dynamic = ProgramHeader {
        p_type: phdr_type::PT_DYNAMIC,
        p_flags: 0,
        p_offset: 0,
        p_vaddr: 0,
        p_paddr: 0,
        p_filesz: 17,
        p_memsz: 17,
        p_align: 8,
    };
    assert!(parse_dynamic_section(&[0; 17], &dynamic, &[], VirtAddr::new(0)).is_err());
}

fn push(buf: &mut Vec<u8>, tag: u64, value: u64) {
    buf.extend_from_slice(&tag.to_le_bytes());
    buf.extend_from_slice(&value.to_le_bytes());
}
