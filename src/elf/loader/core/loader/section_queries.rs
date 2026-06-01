// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use super::state::ElfLoader;
use crate::elf::errors::ElfError;
use crate::elf::types::shdr_type;
use alloc::vec::Vec;

use super::super::section::ParsedSection;

impl ElfLoader {
    pub fn parse_section_headers(&self, elf_data: &[u8]) -> Result<Vec<ParsedSection>, ElfError> {
        super::super::parse_sections::parse_section_headers(elf_data)
    }

    pub fn find_section_by_name<'a>(
        sections: &'a [ParsedSection],
        name: &str,
    ) -> Option<&'a ParsedSection> {
        sections.iter().find(|section| section.name == name)
    }

    pub fn get_symbol_table<'a>(sections: &'a [ParsedSection]) -> Option<&'a ParsedSection> {
        sections.iter().find(|section| section.is_symtab())
    }

    pub fn get_dynsym<'a>(sections: &'a [ParsedSection]) -> Option<&'a ParsedSection> {
        sections.iter().find(|section| section.section_type == shdr_type::SHT_DYNSYM)
    }
}
