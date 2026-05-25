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

use super::table::SectionTable;
use crate::elf::loader::core::parse_dynamic::read_string_from_data_limited;
use crate::elf::types::SectionHeader;
use alloc::string::String;

pub(in crate::elf::loader::core::parse_sections) fn section_name(
    elf_data: &[u8],
    table: &SectionTable,
    header: &SectionHeader,
) -> String {
    let Some((strtab_offset, strtab_size)) = table.string_table else {
        return String::new();
    };
    let Ok(name_index) = usize::try_from(header.sh_name) else {
        return String::new();
    };
    let Some(name_offset) = strtab_offset.checked_add(name_index) else {
        return String::new();
    };
    let Some(strtab_end) = strtab_offset.checked_add(strtab_size) else {
        return String::new();
    };
    if name_offset >= strtab_end {
        return String::new();
    }
    read_string_from_data_limited(elf_data, name_offset, strtab_end - name_offset)
}
