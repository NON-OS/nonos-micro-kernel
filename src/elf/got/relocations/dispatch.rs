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
use crate::elf::types::reloc_type;

use super::{types::ResolvedSymbolValue, write::{copy_symbol, write_u64}};

pub(super) fn dispatch_relocation(rel_type: u32, target_addr: u64, symbol: Option<ResolvedSymbolValue>, addend: i64, base_addr: u64) -> ElfResult<bool> {
    match rel_type {
        reloc_type::R_X86_64_NONE => Ok(false),
        reloc_type::R_X86_64_64 | reloc_type::R_X86_64_TPOFF64 | reloc_type::R_X86_64_DTPOFF64 => match symbol {
            Some(symbol) => { unsafe { write_u64(target_addr, symbol.value.wrapping_add(addend as u64)); } Ok(true) }
            None => Ok(false),
        },
        reloc_type::R_X86_64_GLOB_DAT | reloc_type::R_X86_64_JUMP_SLOT => match symbol {
            Some(symbol) => { unsafe { write_u64(target_addr, symbol.value); } Ok(true) }
            None => Ok(false),
        },
        reloc_type::R_X86_64_RELATIVE => { unsafe { write_u64(target_addr, base_addr.wrapping_add(addend as u64)); } Ok(true) }
        reloc_type::R_X86_64_COPY => match symbol {
            Some(symbol) => { unsafe { copy_symbol(target_addr, symbol.value, symbol.size); } Ok(true) }
            None => Ok(false),
        },
        reloc_type::R_X86_64_DTPMOD64 => { unsafe { write_u64(target_addr, 1); } Ok(true) }
        reloc_type::R_X86_64_IRELATIVE => { unsafe { write_u64(target_addr, base_addr.wrapping_add(addend as u64)); } Ok(true) }
        _ => Err(ElfError::UnsupportedRelocation(rel_type)),
    }
}
