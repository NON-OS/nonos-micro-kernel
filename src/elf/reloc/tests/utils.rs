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

use super::super::utils::{count_supported, count_unsupported, is_supported, reloc_type_name};
use crate::elf::types::{reloc_type, RelaEntry};

#[test]
fn test_reloc_type_name() {
    assert_eq!(reloc_type_name(reloc_type::R_X86_64_NONE), "R_X86_64_NONE");
    assert_eq!(reloc_type_name(reloc_type::R_X86_64_64), "R_X86_64_64");
    assert_eq!(reloc_type_name(reloc_type::R_X86_64_RELATIVE), "R_X86_64_RELATIVE");
    assert_eq!(reloc_type_name(reloc_type::R_X86_64_JUMP_SLOT), "R_X86_64_JUMP_SLOT");
    assert_eq!(reloc_type_name(999), "UNKNOWN");
}

#[test]
fn test_is_supported() {
    assert!(is_supported(reloc_type::R_X86_64_NONE));
    assert!(is_supported(reloc_type::R_X86_64_64));
    assert!(is_supported(reloc_type::R_X86_64_PC32));
    assert!(is_supported(reloc_type::R_X86_64_PLT32));
    assert!(is_supported(reloc_type::R_X86_64_RELATIVE));
    assert!(is_supported(reloc_type::R_X86_64_GLOB_DAT));
    assert!(is_supported(reloc_type::R_X86_64_JUMP_SLOT));
    assert!(is_supported(reloc_type::R_X86_64_32));
    assert!(is_supported(reloc_type::R_X86_64_32S));
    assert!(is_supported(reloc_type::R_X86_64_IRELATIVE));
    assert!(!is_supported(reloc_type::R_X86_64_COPY));
    assert!(!is_supported(reloc_type::R_X86_64_DTPMOD64));
}

#[test]
fn test_count_supported() {
    let entries = [
        RelaEntry { r_offset: 0, r_info: reloc_type::R_X86_64_RELATIVE as u64, r_addend: 0 },
        RelaEntry { r_offset: 8, r_info: reloc_type::R_X86_64_COPY as u64, r_addend: 0 },
        RelaEntry { r_offset: 16, r_info: reloc_type::R_X86_64_64 as u64, r_addend: 0 },
    ];
    assert_eq!(count_supported(&entries), 2);
    assert_eq!(count_unsupported(&entries), 1);
}

#[test]
fn test_rela_entry_methods() {
    let entry = RelaEntry {
        r_offset: 0x1000,
        r_info: (5u64 << 32) | reloc_type::R_X86_64_RELATIVE as u64,
        r_addend: 0x100,
    };
    assert_eq!(entry.reloc_type(), reloc_type::R_X86_64_RELATIVE);
    assert_eq!(entry.symbol_index(), 5);
}
