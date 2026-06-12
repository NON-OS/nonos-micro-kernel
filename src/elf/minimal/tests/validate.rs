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

use super::super::{validate_elf, validate_elf_detailed, validate_elf_x86_64};
use super::support::make_valid_elf_header;
use crate::elf::errors::ElfError;
use crate::elf::types::{elf_class, elf_data};

#[test]
fn test_validate_elf_valid() {
    assert!(validate_elf(&make_valid_elf_header()));
}

#[test]
fn test_validate_elf_too_small() {
    assert!(!validate_elf(&[0u8; 8]));
}

#[test]
fn test_validate_elf_bad_magic() {
    let mut header = make_valid_elf_header();
    header[0] = 0;
    assert!(!validate_elf(&header));
}

#[test]
fn test_validate_elf_bad_class() {
    let mut header = make_valid_elf_header();
    header[4] = elf_class::ELFCLASS32;
    assert!(!validate_elf(&header));
}

#[test]
fn test_validate_elf_bad_endian() {
    let mut header = make_valid_elf_header();
    header[5] = elf_data::ELFDATA2MSB;
    assert!(!validate_elf(&header));
}

#[test]
fn test_validate_elf_detailed() {
    assert!(validate_elf_detailed(&make_valid_elf_header()).is_ok());
}

#[test]
fn test_validate_elf_detailed_bad_version() {
    let mut header = make_valid_elf_header();
    header[6] = 0;
    assert!(matches!(validate_elf_detailed(&header), Err(ElfError::InvalidVersion)));
}

#[test]
fn test_validate_elf_x86_64() {
    assert!(validate_elf_x86_64(&make_valid_elf_header()).is_ok());
}

#[test]
fn test_validate_elf_x86_64_bad_machine() {
    let mut header = make_valid_elf_header();
    header[18] = 0;
    header[19] = 0;
    assert!(matches!(validate_elf_x86_64(&header), Err(ElfError::InvalidMachine)));
}
