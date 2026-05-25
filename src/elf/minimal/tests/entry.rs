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

use super::super::entry_from_bytes;
use super::support::make_valid_elf_header;
use crate::elf::errors::ElfError;

#[test]
fn test_entry_from_bytes_valid() {
    assert_eq!(entry_from_bytes(&make_valid_elf_header()).unwrap(), 0x401000);
}

#[test]
fn test_entry_from_bytes_too_small() {
    assert!(matches!(entry_from_bytes(&[0u8; 32]), Err(ElfError::FileTooSmall)));
}

#[test]
fn test_entry_from_bytes_invalid_magic() {
    let mut header = make_valid_elf_header();
    header[0] = 0;
    assert!(matches!(entry_from_bytes(&header), Err(ElfError::InvalidMagic)));
}

#[test]
fn test_entry_from_bytes_zero_entry() {
    let mut header = make_valid_elf_header();
    header[24..32].copy_from_slice(&0u64.to_le_bytes());
    assert!(matches!(entry_from_bytes(&header), Err(ElfError::Other(_))));
}
