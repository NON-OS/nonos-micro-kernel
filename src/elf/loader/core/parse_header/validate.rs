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
use crate::elf::types::{elf_machine, elf_type, ElfHeader};

pub(crate) fn validate_elf(header: &ElfHeader) -> Result<(), ElfError> {
    if !header.is_valid_magic() {
        return Err(ElfError::InvalidMagic);
    }
    if !header.is_64bit() {
        return Err(ElfError::InvalidClass);
    }
    if !header.is_little_endian() {
        return Err(ElfError::InvalidEndian);
    }
    if !header.version_is_current() {
        return Err(ElfError::InvalidVersion);
    }
    if !header.has_native_header_size() {
        return Err(ElfError::InvalidHeaderSize);
    }
    if !header.has_native_program_header_size() {
        return Err(ElfError::InvalidProgramHeaderSize);
    }
    if !header.has_native_section_header_size() {
        return Err(ElfError::InvalidSectionHeaderSize);
    }
    if !header.has_valid_section_name_table_index() {
        return Err(ElfError::InvalidIndex);
    }
    if header.e_machine != elf_machine::EM_NATIVE {
        return Err(ElfError::InvalidMachine);
    }
    if header.e_type != elf_type::ET_EXEC && header.e_type != elf_type::ET_DYN {
        return Err(ElfError::InvalidType);
    }
    Ok(())
}
