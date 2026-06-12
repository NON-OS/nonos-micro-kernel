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

use core::ptr;

use crate::elf::errors::{ElfError, ElfResult};
use crate::elf::types::{reloc_type, Rela};
use crate::memory::addr::VirtAddr;

use super::{
    dispatch::dispatch_relocation, state::RelocationProcessor, types::ResolvedSymbolValue,
};

impl RelocationProcessor {
    pub fn process_rela(
        &self,
        rela_addr: VirtAddr,
        rela_count: usize,
        symbol_resolver: impl Fn(usize) -> Option<ResolvedSymbolValue>,
    ) -> ElfResult<usize> {
        let mut processed = 0;
        for index in 0..rela_count {
            let rela_ptr =
                (rela_addr.as_u64() + (index * core::mem::size_of::<Rela>()) as u64) as *const Rela;
            let rela = unsafe { ptr::read(rela_ptr) };
            let sym_idx =
                usize::try_from(rela.r_info >> 32).map_err(|_| ElfError::RelocationFailed)?;
            let rel_type = (rela.r_info & 0xFFFF_FFFF) as u32;
            let target_addr = self
                .base_addr
                .as_u64()
                .checked_add(rela.r_offset)
                .ok_or(ElfError::AddressOverflow)?;
            processed += if dispatch_relocation(
                rel_type,
                target_addr,
                symbol_resolver(sym_idx),
                rela.r_addend,
                self.base_addr.as_u64(),
            )? {
                1
            } else {
                0
            };
        }
        Ok(processed)
    }

    pub fn process_plt_relocations(
        &self,
        rela_addr: VirtAddr,
        rela_count: usize,
        symbol_resolver: impl Fn(usize) -> Option<ResolvedSymbolValue>,
    ) -> ElfResult<usize> {
        let mut processed = 0;
        for index in 0..rela_count {
            let rela_ptr =
                (rela_addr.as_u64() + (index * core::mem::size_of::<Rela>()) as u64) as *const Rela;
            let rela = unsafe { ptr::read(rela_ptr) };
            if (rela.r_info & 0xFFFF_FFFF) as u32 != reloc_type::R_X86_64_JUMP_SLOT {
                continue;
            }
            let sym_idx =
                usize::try_from(rela.r_info >> 32).map_err(|_| ElfError::RelocationFailed)?;
            let target_addr = self
                .base_addr
                .as_u64()
                .checked_add(rela.r_offset)
                .ok_or(ElfError::AddressOverflow)?;
            processed += if dispatch_relocation(
                reloc_type::R_X86_64_JUMP_SLOT,
                target_addr,
                symbol_resolver(sym_idx),
                rela.r_addend,
                self.base_addr.as_u64(),
            )? {
                1
            } else {
                0
            };
        }
        Ok(processed)
    }
}
