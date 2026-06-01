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
use crate::elf::types::{phdr_type, ProgramHeader};

pub(in crate::elf::loader::core::parse_dynamic) fn file_offset_for_vaddr(
    program_headers: &[ProgramHeader],
    vaddr: u64,
) -> Result<usize, ElfError> {
    for ph in program_headers {
        let Some(end) = ph.p_vaddr.checked_add(ph.p_filesz) else { continue };
        if ph.p_type != phdr_type::PT_LOAD || vaddr < ph.p_vaddr || vaddr >= end {
            continue;
        }
        let delta = vaddr - ph.p_vaddr;
        let offset = ph.p_offset.checked_add(delta).ok_or(ElfError::StringTableOutOfBounds)?;
        return usize::try_from(offset).map_err(|_| ElfError::StringTableOutOfBounds);
    }
    Err(ElfError::StringTableOutOfBounds)
}
