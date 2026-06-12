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
use crate::elf::errors::ElfError;

pub(super) unsafe fn write_absolute<T>(
    values: RelocationValues,
    cast: fn(i64) -> T,
) -> Result<(), ElfError> {
    unsafe {
        *(values.target_addr as *mut T) =
            cast((values.symbol_value as i64).wrapping_add(values.addend))
    };
    Ok(())
}

pub(super) unsafe fn write_pc_relative<T>(
    values: RelocationValues,
    cast: fn(i64) -> T,
) -> Result<(), ElfError> {
    let value = (values.symbol_value as i64)
        .wrapping_add(values.addend)
        .wrapping_sub(values.target_addr as i64);
    unsafe { *(values.target_addr as *mut T) = cast(value) };
    Ok(())
}

pub(super) unsafe fn write_got<T>(
    values: RelocationValues,
    cast: fn(i64) -> T,
) -> Result<(), ElfError> {
    let got = values.got_base.ok_or(ElfError::UnsupportedRelocation(values.reloc_type))?;
    unsafe {
        *(values.target_addr as *mut T) =
            cast((values.symbol_value.wrapping_sub(got) as i64).wrapping_add(values.addend))
    };
    Ok(())
}

pub(super) unsafe fn write_relative(values: RelocationValues) -> Result<(), ElfError> {
    unsafe {
        *(values.target_addr as *mut u64) = (values.base as i64).wrapping_add(values.addend) as u64
    };
    Ok(())
}

pub(super) unsafe fn write_gotpcrel<T>(
    values: RelocationValues,
    cast: fn(i64) -> T,
) -> Result<(), ElfError> {
    let got = values.got_base.ok_or(ElfError::UnsupportedRelocation(values.reloc_type))?;
    unsafe {
        *(values.target_addr as *mut T) =
            cast((got as i64).wrapping_add(values.addend).wrapping_sub(values.target_addr as i64))
    };
    Ok(())
}

pub(super) fn write_copy(values: RelocationValues) -> Result<(), ElfError> {
    if values.symbol_value != 0 {
        return Err(ElfError::UnsupportedRelocation(values.reloc_type));
    }
    Ok(())
}

pub(super) unsafe fn write_irelative(values: RelocationValues) -> Result<(), ElfError> {
    let resolver_addr = values.resolver_addr.ok_or(ElfError::InvalidState)?;
    let resolver: extern "C" fn() -> u64 =
        unsafe { core::mem::transmute(resolver_addr as *const ()) };
    unsafe { *(values.target_addr as *mut u64) = resolver() };
    Ok(())
}
