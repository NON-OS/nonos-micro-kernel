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

use super::state::RelocationValues;
use super::write::{
    write_absolute, write_copy, write_got, write_gotpcrel, write_irelative, write_pc_relative,
    write_relative,
};
use crate::elf::errors::ElfError;
use crate::elf::types::reloc_type;

pub(super) unsafe fn apply(values: RelocationValues) -> Result<(), ElfError> {
    match values.reloc_type {
        reloc_type::R_X86_64_NONE => Ok(()),
        reloc_type::R_X86_64_64 => unsafe { write_absolute::<u64>(values, |v| v as u64) },
        reloc_type::R_X86_64_PC32 => unsafe { write_pc_relative::<i32>(values, |v| v as i32) },
        reloc_type::R_X86_64_GOT32 => unsafe { write_got::<i32>(values, |v| v as i32) },
        reloc_type::R_X86_64_PLT32 => unsafe { write_pc_relative::<i32>(values, |v| v as i32) },
        reloc_type::R_X86_64_COPY => write_copy(values),
        reloc_type::R_X86_64_GLOB_DAT => unsafe { write_absolute::<u64>(values, |v| v as u64) },
        reloc_type::R_X86_64_JUMP_SLOT => unsafe { write_absolute::<u64>(values, |v| v as u64) },
        reloc_type::R_X86_64_RELATIVE => unsafe { write_relative(values) },
        reloc_type::R_X86_64_GOTPCREL => unsafe { write_gotpcrel::<i32>(values, |v| v as i32) },
        reloc_type::R_X86_64_32 => unsafe { write_absolute::<u32>(values, |v| v as u32) },
        reloc_type::R_X86_64_32S => unsafe { write_absolute::<i32>(values, |v| v as i32) },
        reloc_type::R_X86_64_16 => unsafe { write_absolute::<u16>(values, |v| v as u16) },
        reloc_type::R_X86_64_PC16 => unsafe { write_pc_relative::<i16>(values, |v| v as i16) },
        reloc_type::R_X86_64_8 => unsafe { write_absolute::<u8>(values, |v| v as u8) },
        reloc_type::R_X86_64_PC8 => unsafe { write_pc_relative::<i8>(values, |v| v as i8) },
        reloc_type::R_X86_64_IRELATIVE => unsafe { write_irelative(values) },
        _ => Err(ElfError::UnsupportedRelocation(values.reloc_type)),
    }
}
