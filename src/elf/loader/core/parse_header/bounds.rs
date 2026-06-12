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
use crate::elf::types::{ElfHeader, ProgramHeader};
use core::mem::size_of;

pub(crate) fn program_header_bounds(
    elf_data: &[u8],
    header: &ElfHeader,
) -> Result<(usize, usize, usize), ElfError> {
    let ph_offset =
        usize::try_from(header.e_phoff).map_err(|_| ElfError::ProgramHeadersOutOfBounds)?;
    let ph_size = usize::from(header.e_phentsize);
    let ph_count = usize::from(header.e_phnum);
    if ph_count == 0 {
        return Ok((ph_offset, ph_size, ph_count));
    }
    if ph_size != size_of::<ProgramHeader>() {
        return Err(ElfError::InvalidProgramHeaderSize);
    }
    let table_bytes = ph_size.checked_mul(ph_count).ok_or(ElfError::ProgramHeadersOutOfBounds)?;
    let table_end =
        ph_offset.checked_add(table_bytes).ok_or(ElfError::ProgramHeadersOutOfBounds)?;
    if table_end > elf_data.len() {
        return Err(ElfError::ProgramHeadersOutOfBounds);
    }
    Ok((ph_offset, ph_size, ph_count))
}
