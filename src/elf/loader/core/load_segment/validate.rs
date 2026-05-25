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

use crate::elf::errors::ElfError;
use crate::elf::types::ProgramHeader;

pub(super) fn input_fields(
    elf_len: usize,
    header: &ProgramHeader,
) -> Result<(usize, usize, usize, usize, usize), ElfError> {
    if header.p_filesz > header.p_memsz { return Err(ElfError::SegmentDataOutOfBounds); }
    if header.is_writable() && header.is_executable() { return Err(ElfError::WXViolation); }
    if header.p_align > 1 && !header.p_align.is_power_of_two() { return Err(ElfError::AlignmentError); }
    if header.p_align > 1 && header.p_vaddr % header.p_align != header.p_offset % header.p_align {
        return Err(ElfError::AlignmentError);
    }
    let file_size = usize::try_from(header.p_filesz).map_err(|_| ElfError::SegmentDataOutOfBounds)?;
    let seg_size = usize::try_from(header.p_memsz).map_err(|_| ElfError::SegmentDataOutOfBounds)?;
    let file_offset = usize::try_from(header.p_offset).map_err(|_| ElfError::SegmentDataOutOfBounds)?;
    let intra = usize::try_from(header.p_vaddr & 0xFFF).map_err(|_| ElfError::InvalidAddress)?;
    let file_end = file_offset.checked_add(file_size).ok_or(ElfError::SegmentDataOutOfBounds)?;
    if file_end > elf_len { return Err(ElfError::SegmentDataOutOfBounds); }
    Ok((file_offset, file_size, file_end, seg_size, intra))
}
