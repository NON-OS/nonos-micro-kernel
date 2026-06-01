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

use super::bounds::program_header_bounds;
use super::program_entry::parse_program_header_at;
use crate::elf::errors::ElfError;
use crate::elf::types::{ElfHeader, ProgramHeader};
use alloc::vec::Vec;

pub(crate) fn parse_program_headers(
    elf_data: &[u8],
    header: &ElfHeader,
) -> Result<Vec<ProgramHeader>, ElfError> {
    let (_, _, ph_count) = program_header_bounds(elf_data, header)?;
    let mut program_headers = Vec::with_capacity(ph_count);
    for index in 0..ph_count {
        program_headers.push(parse_program_header_at(elf_data, header, index)?);
    }
    Ok(program_headers)
}
