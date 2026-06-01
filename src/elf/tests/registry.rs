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

use crate::test::framework::{TestCase, TestSuite};

use super::smoke::*;

pub fn run_all() -> bool {
    let mut suite = TestSuite::new("ELF");
    suite.add(TestCase::with_category("elf_magic_constant", test_elf_magic_constant, "elf"));
    suite.add(TestCase::with_category("elf_header_size", test_elf_header_size, "elf"));
    suite.add(TestCase::with_category("program_header_size", test_program_header_size, "elf"));
    suite.add(TestCase::with_category("section_header_size", test_section_header_size, "elf"));
    suite.add(TestCase::with_category("aslr_manager_creation", test_aslr_manager_creation, "elf"));
    suite.add(TestCase::with_category("aslr_disabled", test_aslr_disabled, "elf"));
    suite.add(TestCase::with_category("elf_class_values", test_elf_class_values, "elf"));
    suite.add(TestCase::with_category("elf_type_values", test_elf_type_values, "elf"));
    suite.add(TestCase::with_category("elf_machine_values", test_elf_machine_values, "elf"));
    suite.add(TestCase::with_category("phdr_type_values", test_phdr_type_values, "elf"));
    suite.add(TestCase::with_category("validate_elf_magic", test_validate_elf_magic, "elf"));
    let (_, failed, _) = suite.run_all();
    failed == 0
}
