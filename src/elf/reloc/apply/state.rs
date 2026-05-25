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

use crate::elf::errors::ElfError;
use crate::elf::loader::ElfImage;
use crate::elf::types::{reloc_type, RelaEntry};

use super::bounds::{ensure_target_range, resolve_resolver_addr};
use super::super::context::RelocationContext;

#[derive(Clone, Copy)]
pub(super) struct RelocationValues {
    pub target_addr: u64,
    pub reloc_type: u32,
    pub addend: i64,
    pub base: u64,
    pub symbol_value: u64,
    pub got_base: Option<u64>,
    pub resolver_addr: Option<u64>,
}

impl RelocationValues {
    pub(super) fn resolve(
        image: &ElfImage,
        rela: &RelaEntry,
        context: &RelocationContext,
    ) -> Result<Self, ElfError> {
        let target_addr = image
            .base_addr
            .as_u64()
            .checked_add(rela.r_offset)
            .ok_or(ElfError::AddressOverflow)?;
        let reloc_type = rela.reloc_type();
        ensure_target_range(image, target_addr, reloc_type)?;
        let symbol_value = context
            .resolve_symbol(rela.symbol_index(), image.base_addr)
            .unwrap_or(0);
        let resolver_addr = if reloc_type == reloc_type::R_X86_64_IRELATIVE {
            Some(resolve_resolver_addr(image, image.base_addr.as_u64(), rela.r_addend)?)
        } else {
            None
        };
        Ok(Self {
            target_addr,
            reloc_type,
            addend: rela.r_addend,
            base: image.base_addr.as_u64(),
            symbol_value,
            got_base: context.got_base.map(|addr| addr.as_u64()),
            resolver_addr,
        })
    }
}
