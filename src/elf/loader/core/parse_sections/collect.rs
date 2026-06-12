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

use super::names::section_name;
use super::table::section_table;
use crate::elf::errors::ElfError;
use crate::elf::loader::core::parse_header::parse_elf_header;
use crate::elf::loader::core::section::ParsedSection;
use crate::elf::types::{shdr_type, SectionHeader};
use alloc::vec::Vec;
use core::ptr;

pub(in crate::elf::loader::core) fn parse_section_headers(
    elf_data: &[u8],
) -> Result<Vec<ParsedSection>, ElfError> {
    let header = parse_elf_header(elf_data)?;
    let Some(table) = section_table(elf_data, &header)? else {
        return Ok(Vec::new());
    };
    let mut sections = Vec::with_capacity(table.count);
    for index in 0..table.count {
        let off = table.offset + index * table.entry_size;
        let header =
            unsafe { ptr::read_unaligned(elf_data[off..].as_ptr() as *const SectionHeader) };
        if header.sh_type != shdr_type::SHT_NOBITS {
            let end = header
                .sh_offset
                .checked_add(header.sh_size)
                .ok_or(ElfError::SectionHeadersOutOfBounds)?;
            if end > elf_data.len() as u64 {
                return Err(ElfError::SectionHeadersOutOfBounds);
            }
        }
        sections.push(ParsedSection {
            name: section_name(elf_data, &table, &header),
            section_type: header.sh_type,
            flags: header.sh_flags,
            addr: header.sh_addr,
            offset: header.sh_offset,
            size: header.sh_size,
            link: header.sh_link,
            info: header.sh_info,
            alignment: header.sh_addralign,
            entry_size: header.sh_entsize,
        });
    }
    Ok(sections)
}
