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

use crate::elf::errors::ElfResult;
use crate::elf::types::reloc_type;
use crate::memory::addr::VirtAddr;

use super::{dispatch::dispatch_relocation, types::ResolvedSymbolValue};

pub fn apply_single_relocation(
    target: VirtAddr,
    rel_type: u32,
    symbol_value: u64,
    addend: i64,
    base_addr: VirtAddr,
) -> ElfResult<()> {
    if rel_type != reloc_type::R_X86_64_NONE
        && rel_type != reloc_type::R_X86_64_64
        && rel_type != reloc_type::R_X86_64_GLOB_DAT
        && rel_type != reloc_type::R_X86_64_JUMP_SLOT
        && rel_type != reloc_type::R_X86_64_RELATIVE
        && rel_type != reloc_type::R_X86_64_TPOFF64
    {
        return dispatch_relocation(rel_type, target.as_u64(), None, addend, base_addr.as_u64())
            .map(|_| ());
    }
    let symbol = ResolvedSymbolValue { value: symbol_value, size: 0 };
    dispatch_relocation(rel_type, target.as_u64(), Some(symbol), addend, base_addr.as_u64())
        .map(|_| ())
}
