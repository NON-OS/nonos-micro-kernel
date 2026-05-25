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

use super::state::SectionHeader;
use crate::elf::types::shdr_type;

impl SectionHeader {
    pub fn type_name(&self) -> &'static str {
        match self.sh_type {
            shdr_type::SHT_NULL => "NULL",
            shdr_type::SHT_PROGBITS => "PROGBITS",
            shdr_type::SHT_SYMTAB => "SYMTAB",
            shdr_type::SHT_STRTAB => "STRTAB",
            shdr_type::SHT_RELA => "RELA",
            shdr_type::SHT_HASH => "HASH",
            shdr_type::SHT_DYNAMIC => "DYNAMIC",
            shdr_type::SHT_NOTE => "NOTE",
            shdr_type::SHT_NOBITS => "NOBITS",
            shdr_type::SHT_REL => "REL",
            shdr_type::SHT_DYNSYM => "DYNSYM",
            _ => "UNKNOWN",
        }
    }
}
