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

use crate::elf::errors::ElfError;
use crate::elf::types::{ElfHeader, SectionHeader};
use core::ptr;
use core::mem::size_of;

pub(in crate::elf::loader::core::parse_sections) struct SectionTable {
    pub offset: usize,
    pub entry_size: usize,
    pub count: usize,
    pub string_table: Option<(usize, usize)>,
}

pub(in crate::elf::loader::core::parse_sections) fn section_table(
    elf_data: &[u8],
    header: &ElfHeader,
) -> Result<Option<SectionTable>, ElfError> {
    if header.e_shoff == 0 || header.e_shnum == 0 {
        return Ok(None);
    }
    let offset = usize::try_from(header.e_shoff).map_err(|_| ElfError::SectionHeadersOutOfBounds)?;
    let entry_size = usize::from(header.e_shentsize);
    let count = usize::from(header.e_shnum);
    if entry_size != size_of::<SectionHeader>() {
        return Err(ElfError::InvalidSectionHeaderSize);
    }
    let table_bytes = entry_size.checked_mul(count).ok_or(ElfError::SectionHeadersOutOfBounds)?;
    let table_end = offset.checked_add(table_bytes).ok_or(ElfError::SectionHeadersOutOfBounds)?;
    if table_end > elf_data.len() {
        return Err(ElfError::SectionHeadersOutOfBounds);
    }
    let string_table = string_table(elf_data, offset, entry_size, count, usize::from(header.e_shstrndx))?;
    Ok(Some(SectionTable { offset, entry_size, count, string_table }))
}

fn string_table(
    elf_data: &[u8],
    offset: usize,
    entry_size: usize,
    count: usize,
    string_index: usize,
) -> Result<Option<(usize, usize)>, ElfError> {
    if string_index >= count || string_index == 0 {
        return Ok(None);
    }
    let off = offset.checked_add(string_index.checked_mul(entry_size).ok_or(ElfError::SectionHeadersOutOfBounds)?).ok_or(ElfError::SectionHeadersOutOfBounds)?;
    let header = unsafe { ptr::read_unaligned(elf_data[off..].as_ptr() as *const SectionHeader) };
    let table_offset = usize::try_from(header.sh_offset).map_err(|_| ElfError::StringTableOutOfBounds)?;
    let table_size = usize::try_from(header.sh_size).map_err(|_| ElfError::StringTableOutOfBounds)?;
    let table_end = table_offset.checked_add(table_size).ok_or(ElfError::StringTableOutOfBounds)?;
    if table_end > elf_data.len() {
        return Err(ElfError::StringTableOutOfBounds);
    }
    Ok(Some((table_offset, table_size)))
}
