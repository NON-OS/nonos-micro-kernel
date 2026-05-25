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

use super::state::ProgramHeader;
use crate::elf::types::{phdr_flags, phdr_type};
use core::mem;

#[test]
fn test_program_header_size() {
    assert_eq!(mem::size_of::<ProgramHeader>(), ProgramHeader::SIZE);
}

#[test]
fn test_program_header_flags() {
    let mut ph = ProgramHeader::default();
    ph.p_type = phdr_type::PT_LOAD;
    ph.p_flags = phdr_flags::PF_R | phdr_flags::PF_X;
    assert!(ph.is_load());
    assert!(ph.is_readable());
    assert!(!ph.is_writable());
    assert!(ph.is_executable());
    assert_eq!(ph.flags_str(), "R-X");
}

#[test]
fn test_program_header_bss() {
    let mut ph = ProgramHeader::default();
    ph.p_filesz = 0x1000;
    ph.p_memsz = 0x2000;
    assert_eq!(ph.bss_size(), 0x1000);
}
