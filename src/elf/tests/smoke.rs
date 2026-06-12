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

use crate::elf::*;
use crate::test::framework::TestResult;

pub(crate) fn test_elf_magic_constant() -> TestResult {
    if ELF_MAGIC == [0x7f, b'E', b'L', b'F'] {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
pub(crate) fn test_elf_header_size() -> TestResult {
    if core::mem::size_of::<ElfHeader>() == 64 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
pub(crate) fn test_program_header_size() -> TestResult {
    if core::mem::size_of::<ProgramHeader>() == 56 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
pub(crate) fn test_section_header_size() -> TestResult {
    if core::mem::size_of::<SectionHeader>() == 64 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_aslr_manager_creation() -> TestResult {
    let manager = AslrManager::new();
    if manager.is_executable_randomization_enabled()
        && manager.is_stack_randomization_enabled()
        && manager.is_heap_randomization_enabled()
    {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_aslr_disabled() -> TestResult {
    let manager = AslrManager::disabled();
    if !manager.is_executable_randomization_enabled()
        && !manager.is_stack_randomization_enabled()
        && !manager.is_heap_randomization_enabled()
    {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_elf_class_values() -> TestResult {
    if elf_class::NONE == 0 && elf_class::CLASS32 == 1 && elf_class::CLASS64 == 2 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_elf_type_values() -> TestResult {
    if elf_type::NONE == 0
        && elf_type::REL == 1
        && elf_type::EXEC == 2
        && elf_type::DYN == 3
        && elf_type::CORE == 4
    {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_elf_machine_values() -> TestResult {
    if elf_machine::NONE == 0 && elf_machine::X86_64 == 62 {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_phdr_type_values() -> TestResult {
    if phdr_type::NULL == 0
        && phdr_type::LOAD == 1
        && phdr_type::DYNAMIC == 2
        && phdr_type::INTERP == 3
        && phdr_type::NOTE == 4
        && phdr_type::PHDR == 6
        && phdr_type::TLS == 7
    {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}

pub(crate) fn test_validate_elf_magic() -> TestResult {
    let valid = [0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let invalid = [0x00, 0x00, 0x00, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    if validate_elf(&valid) && !validate_elf(&invalid) {
        TestResult::Pass
    } else {
        TestResult::Fail
    }
}
