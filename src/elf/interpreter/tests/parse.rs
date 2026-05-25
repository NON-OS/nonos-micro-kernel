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

use alloc::vec;

use super::super::InterpreterInfo;
use crate::elf::errors::ElfError;
use crate::elf::types::{phdr_type, ProgramHeader};

fn program_header(offset: u64, size: u64) -> ProgramHeader {
    ProgramHeader { p_type: phdr_type::PT_INTERP, p_flags: 0, p_offset: offset, p_vaddr: 0, p_paddr: 0, p_filesz: size, p_memsz: size, p_align: 1 }
}

#[test]
fn test_from_elf_valid() {
    let mut elf_data = vec![0u8; 100];
    let path = b"/lib64/ld-linux-x86-64.so.2\0";
    elf_data[10..10 + path.len()].copy_from_slice(path);
    let info = InterpreterInfo::from_elf(&elf_data, &program_header(10, path.len() as u64)).unwrap();
    assert_eq!(info.path, "/lib64/ld-linux-x86-64.so.2");
}

#[test]
fn test_from_elf_out_of_bounds() {
    let result = InterpreterInfo::from_elf(&vec![0u8; 10], &program_header(5, 20));
    assert!(matches!(result, Err(ElfError::InterpreterNotFound)));
}

#[test]
fn test_from_elf_empty() {
    let result = InterpreterInfo::from_elf(&vec![0u8; 100], &program_header(10, 0));
    assert!(matches!(result, Err(ElfError::InterpreterNotFound)));
}

#[test]
fn test_from_elf_wrong_program_type() {
    let mut ph = program_header(0, 4);
    ph.p_type = phdr_type::PT_LOAD;
    let result = InterpreterInfo::from_elf(&[b'/', b'l', b'd', 0], &ph);
    assert!(matches!(result, Err(ElfError::InterpreterNotFound)));
}

#[test]
fn test_from_elf_requires_nul_terminator() {
    let result = InterpreterInfo::from_elf(b"/lib64/ld-linux-x86-64.so.2", &program_header(0, 27));
    assert!(matches!(result, Err(ElfError::Other("PT_INTERP missing NUL terminator"))));
}
