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

pub use super::context::ValidationContext;
pub use super::header::{validate_entry_point, validate_header, validate_ident, validate_magic};
pub use super::program::{validate_program_header, validate_program_headers};

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::Elf64Header;

pub fn validate_elf(data: &[u8]) -> LoaderResult<ValidationContext> {
    if data.len() < core::mem::size_of::<Elf64Header>() {
        return Err(LoaderError::ElfParseError("file too small"));
    }

    validate_magic(data)?;

    let (is_64bit, _is_little_endian) = validate_ident(data)?;
    if !is_64bit {
        return Err(LoaderError::InvalidClass);
    }

    // SAFETY: We've validated minimum size
    let header = unsafe { &*(data.as_ptr() as *const Elf64Header) };
    let mut ctx = validate_header(header)?;

    let _segments = validate_program_headers(data, header, &mut ctx)?;

    validate_entry_point(&ctx)?;

    Ok(ctx)
}

pub fn validate_elf_strict(data: &[u8]) -> LoaderResult<ValidationContext> {
    let ctx = validate_elf(data)?;

    if ctx.wx_segments > 0 {
        return Err(LoaderError::WxViolation);
    }

    if ctx.is_pie && !ctx.has_dynamic {
        return Err(LoaderError::MalformedElf("PIE without dynamic section"));
    }

    Ok(ctx)
}

