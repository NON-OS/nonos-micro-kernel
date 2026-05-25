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

use super::state::DynamicParseState;
use crate::elf::errors::ElfError;
use crate::elf::types::DynamicEntry;
use crate::memory::addr::VirtAddr;
use core::ptr;

pub(in crate::elf::loader::core::parse_dynamic) fn parse_dynamic_entry(
    elf_data: &[u8],
    entry_offset: usize,
    base_addr: VirtAddr,
    state: &mut DynamicParseState,
) -> Result<bool, ElfError> {
    let entry = unsafe { ptr::read_unaligned(elf_data[entry_offset..].as_ptr() as *const DynamicEntry) };
    match entry.d_tag {
        0 => Ok(false),
        1 => {
            state.needed_offsets.push(entry.value);
            Ok(true)
        }
        5 => {
            state.dynamic_info.string_table = Some(dynamic_addr(base_addr, entry.value)?);
            Ok(true)
        }
        10 => assign_size(&mut state.dynamic_info.string_table_size, entry.value),
        6 => {
            state.dynamic_info.symbol_table = Some(dynamic_addr(base_addr, entry.value)?);
            Ok(true)
        }
        7 => {
            state.dynamic_info.rela_table = Some(dynamic_addr(base_addr, entry.value)?);
            Ok(true)
        }
        8 => assign_size(&mut state.dynamic_info.rela_size, entry.value),
        23 => {
            state.dynamic_info.plt_relocations = Some(dynamic_addr(base_addr, entry.value)?);
            Ok(true)
        }
        2 => assign_size(&mut state.dynamic_info.plt_rela_size, entry.value),
        12 => {
            state.dynamic_info.init_function = Some(dynamic_addr(base_addr, entry.value)?);
            Ok(true)
        }
        13 => {
            state.dynamic_info.fini_function = Some(dynamic_addr(base_addr, entry.value)?);
            Ok(true)
        }
        _ => Ok(true),
    }
}

fn dynamic_addr(base_addr: VirtAddr, value: u64) -> Result<VirtAddr, ElfError> {
    Ok(VirtAddr::new(base_addr.as_u64().checked_add(value).ok_or(ElfError::AddressOverflow)?))
}

fn assign_size(slot: &mut usize, value: u64) -> Result<bool, ElfError> {
    *slot = usize::try_from(value).map_err(|_| ElfError::DynamicSectionError)?;
    Ok(true)
}
