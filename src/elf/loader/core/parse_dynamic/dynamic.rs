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

use super::entry::parse_dynamic_entry;
use super::finalize::append_needed_libraries;
use super::offset::file_offset_for_vaddr;
use super::state::DynamicParseState;
use crate::elf::errors::ElfError;
use crate::elf::loader::image::DynamicInfo;
use crate::elf::types::{DynamicEntry, ProgramHeader};
use crate::memory::addr::VirtAddr;

pub(in crate::elf::loader::core) fn parse_dynamic_section(
    elf_data: &[u8],
    ph: &ProgramHeader,
    program_headers: &[ProgramHeader],
    base_addr: VirtAddr,
) -> Result<DynamicInfo, ElfError> {
    let (section_offset, entry_count) = section_range(ph, elf_data.len())?;
    let mut state = DynamicParseState::new();
    for index in 0..entry_count {
        let entry_offset = section_offset + (index * DynamicEntry::SIZE);
        if !parse_dynamic_entry(elf_data, entry_offset, base_addr, &mut state)? {
            break;
        }
    }
    if !state.needed_offsets.is_empty() {
        let strtab = state.dynamic_info.string_table.ok_or(ElfError::StringTableError)?;
        let relative = strtab
            .as_u64()
            .checked_sub(base_addr.as_u64())
            .ok_or(ElfError::StringTableOutOfBounds)?;
        let strtab_offset = file_offset_for_vaddr(program_headers, relative)?;
        append_needed_libraries(
            elf_data,
            strtab_offset,
            state.dynamic_info.string_table_size,
            &state.needed_offsets,
            &mut state.dynamic_info,
        )?;
    }
    Ok(state.dynamic_info)
}

fn section_range(ph: &ProgramHeader, elf_len: usize) -> Result<(usize, usize), ElfError> {
    let offset = usize::try_from(ph.p_offset).map_err(|_| ElfError::DynamicSectionError)?;
    let size = usize::try_from(ph.p_filesz).map_err(|_| ElfError::DynamicSectionError)?;
    if size % DynamicEntry::SIZE != 0 {
        return Err(ElfError::DynamicSectionError);
    }
    let end = offset.checked_add(size).ok_or(ElfError::DynamicSectionError)?;
    if end > elf_len {
        return Err(ElfError::DynamicSectionError);
    }
    Ok((offset, size / DynamicEntry::SIZE))
}
