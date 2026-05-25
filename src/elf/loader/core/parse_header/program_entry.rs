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

use super::bounds::program_header_bounds;
use crate::elf::errors::ElfError;
use crate::elf::types::{ElfHeader, ProgramHeader};
use core::ptr;

pub(crate) fn parse_program_header_at(
    elf_data: &[u8],
    header: &ElfHeader,
    index: usize,
) -> Result<ProgramHeader, ElfError> {
    let (ph_offset, ph_size, ph_count) = program_header_bounds(elf_data, header)?;
    if index >= ph_count {
        return Err(ElfError::ProgramHeadersOutOfBounds);
    }
    let off = ph_offset
        .checked_add(
            ph_size
                .checked_mul(index)
                .ok_or(ElfError::ProgramHeadersOutOfBounds)?,
        )
        .ok_or(ElfError::ProgramHeadersOutOfBounds)?;
    unsafe { Ok(ptr::read_unaligned(elf_data[off..].as_ptr() as *const ProgramHeader)) }
}
