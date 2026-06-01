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
use super::string::read_string_from_data_limited;
use crate::elf::loader::image::DynamicInfo;

pub(in crate::elf::loader::core::parse_dynamic) fn append_needed_libraries(
    elf_data: &[u8],
    strtab_offset: usize,
    strtab_size: usize,
    needed_offsets: &[u64],
    dynamic_info: &mut DynamicInfo,
) -> Result<(), ElfError> {
    let strtab_end = strtab_offset.checked_add(strtab_size).ok_or(ElfError::StringTableOutOfBounds)?;
    if strtab_end > elf_data.len() { return Err(ElfError::StringTableOutOfBounds); }
    for &name_offset in needed_offsets {
        let name_offset = usize::try_from(name_offset).map_err(|_| ElfError::StringTableOutOfBounds)?;
        let string_offset = strtab_offset.checked_add(name_offset).ok_or(ElfError::StringTableOutOfBounds)?;
        if string_offset >= strtab_end { return Err(ElfError::StringTableOutOfBounds); }
        if !elf_data[string_offset..strtab_end].contains(&0) { return Err(ElfError::StringTableError); }
        let name = read_string_from_data_limited(elf_data, string_offset, strtab_end - string_offset);
        if name.is_empty() { return Err(ElfError::StringTableError); }
        dynamic_info.needed_libraries.push(name);
    }
    Ok(())
}
