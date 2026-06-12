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

use crate::elf::errors::{ElfError, ElfResult};
use crate::elf::loader::{parse_elf_header, parse_program_headers, validate_elf, ElfImage};
use crate::elf::types::{phdr_type, ProgramHeader};
use crate::memory::addr::VirtAddr;

use super::state::ProcessBuilder;

impl<'a> ProcessBuilder<'a> {
    pub(super) fn find_phdr_addr(&self, image: &ElfImage, elf_data: &[u8]) -> ElfResult<VirtAddr> {
        let header = parse_elf_header(elf_data)?;
        validate_elf(&header)?;
        let phdrs = parse_program_headers(elf_data, &header)?;
        let span = phdr_span(&header)?;
        for (segment, header) in image.segments.iter().zip(phdrs.iter().filter(is_load)) {
            let seg_file_end =
                header.p_offset.checked_add(header.p_filesz).ok_or(ElfError::AddressOverflow)?;
            if span.0 >= header.p_offset && span.1 <= seg_file_end {
                return Ok(segment.vaddr + (span.0 - header.p_offset));
            }
        }
        Err(ElfError::ProgramHeadersOutOfBounds)
    }

    pub(super) fn get_phnum(&self, elf_data: &[u8]) -> ElfResult<u16> {
        let header = parse_elf_header(elf_data)?;
        validate_elf(&header)?;
        Ok(header.e_phnum)
    }
}

fn is_load(header: &&ProgramHeader) -> bool {
    header.p_type == phdr_type::PT_LOAD
}

fn phdr_span(header: &crate::elf::types::ElfHeader) -> ElfResult<(u64, u64)> {
    let table_bytes = (header.e_phentsize as u64)
        .checked_mul(header.e_phnum as u64)
        .ok_or(ElfError::AddressOverflow)?;
    let table_end = header.e_phoff.checked_add(table_bytes).ok_or(ElfError::AddressOverflow)?;
    Ok((header.e_phoff, table_end))
}
