// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::{
    elf_class, elf_data, elf_machine, elf_type, Elf64Header, Elf64Phdr, ELF_MAGIC,
};

use super::context::ValidationContext;

pub fn validate_magic(data: &[u8]) -> LoaderResult<()> {
    if data.len() < 4 {
        return Err(LoaderError::ElfParseError("file too small for magic"));
    }

    if data[0..4] != ELF_MAGIC {
        return Err(LoaderError::InvalidMagic);
    }

    Ok(())
}

pub fn validate_ident(data: &[u8]) -> LoaderResult<(bool, bool)> {
    if data.len() < 16 {
        return Err(LoaderError::ElfParseError("file too small for ident"));
    }

    let is_64bit = match data[4] {
        elf_class::ELFCLASS64 => true,
        elf_class::ELFCLASS32 => false,
        _ => return Err(LoaderError::InvalidClass),
    };

    let is_little_endian = match data[5] {
        elf_data::ELFDATA2LSB => true,
        elf_data::ELFDATA2MSB => false,
        _ => return Err(LoaderError::InvalidEndian),
    };

    if data[6] != 1 {
        return Err(LoaderError::InvalidVersion);
    }

    Ok((is_64bit, is_little_endian))
}

pub fn validate_header(header: &Elf64Header) -> LoaderResult<ValidationContext> {
    let mut ctx = ValidationContext::default();

    if !header.is_valid() {
        return Err(LoaderError::InvalidMagic);
    }

    ctx.is_64bit = true;
    ctx.is_little_endian = true;

    match header.e_type {
        elf_type::ET_EXEC => {
            ctx.is_executable = true;
            ctx.is_pie = false;
        }
        elf_type::ET_DYN => {
            return Err(LoaderError::UnsupportedElf("ET_DYN forbidden; kernel must be ET_EXEC"));
        }
        _ => return Err(LoaderError::UnsupportedElf("not executable or shared object")),
    }

    if header.e_machine != elf_machine::EM_X86_64 {
        return Err(LoaderError::UnsupportedElf("not x86_64"));
    }
    ctx.machine = header.e_machine;

    if header.e_entry == 0 && !ctx.is_pie {
        return Err(LoaderError::UnsupportedElf("zero entry point"));
    }
    ctx.entry_point = header.e_entry;

    if header.e_phentsize as usize != core::mem::size_of::<Elf64Phdr>() {
        return Err(LoaderError::MalformedElf("invalid phentsize"));
    }

    Ok(ctx)
}

pub fn validate_entry_point(ctx: &ValidationContext) -> LoaderResult<()> {
    if ctx.is_pie {
        return Ok(());
    }

    if ctx.entry_point < ctx.min_addr || ctx.entry_point >= ctx.max_addr {
        return Err(LoaderError::EntryNotInRange);
    }

    Ok(())
}
