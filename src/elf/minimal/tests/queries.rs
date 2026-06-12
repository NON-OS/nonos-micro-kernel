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

use super::super::{get_elf_machine, get_elf_type, get_phnum, get_phoff, is_pie};
use super::support::make_valid_elf_header;
use crate::elf::types::{elf_machine, elf_type};

#[test]
fn test_get_elf_type() {
    assert_eq!(get_elf_type(&make_valid_elf_header()).unwrap(), elf_type::ET_EXEC);
}

#[test]
fn test_get_elf_machine() {
    assert_eq!(get_elf_machine(&make_valid_elf_header()).unwrap(), elf_machine::EM_X86_64);
}

#[test]
fn test_is_pie() {
    assert!(!is_pie(&make_valid_elf_header()).unwrap());
    let mut header = make_valid_elf_header();
    header[16] = (elf_type::ET_DYN & 0xFF) as u8;
    header[17] = ((elf_type::ET_DYN >> 8) & 0xFF) as u8;
    assert!(is_pie(&header).unwrap());
}

#[test]
fn test_get_phoff() {
    assert_eq!(get_phoff(&make_valid_elf_header()).unwrap(), 64);
}

#[test]
fn test_get_phnum() {
    assert_eq!(get_phnum(&make_valid_elf_header()).unwrap(), 3);
}
