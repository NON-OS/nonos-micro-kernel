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
use crate::elf::types::reloc_type;

pub(super) fn ensure_target_range(
    image: &ElfImage,
    addr: u64,
    reloc_type: u32,
) -> Result<(), ElfError> {
    let size = match reloc_type {
        reloc_type::R_X86_64_NONE | reloc_type::R_X86_64_COPY => 0,
        reloc_type::R_X86_64_64
        | reloc_type::R_X86_64_GLOB_DAT
        | reloc_type::R_X86_64_JUMP_SLOT
        | reloc_type::R_X86_64_RELATIVE
        | reloc_type::R_X86_64_IRELATIVE => 8,
        reloc_type::R_X86_64_PC32
        | reloc_type::R_X86_64_GOT32
        | reloc_type::R_X86_64_PLT32
        | reloc_type::R_X86_64_GOTPCREL
        | reloc_type::R_X86_64_32
        | reloc_type::R_X86_64_32S => 4,
        reloc_type::R_X86_64_16 | reloc_type::R_X86_64_PC16 => 2,
        reloc_type::R_X86_64_8 | reloc_type::R_X86_64_PC8 => 1,
        other => return Err(ElfError::UnsupportedRelocation(other)),
    };
    if size == 0 || in_segment(image, addr, size, false) {
        return Ok(());
    }
    Err(ElfError::RelocationFailed)
}

pub(super) fn resolve_resolver_addr(
    image: &ElfImage,
    base: u64,
    addend: i64,
) -> Result<u64, ElfError> {
    let addr = base.checked_add_signed(addend).ok_or(ElfError::AddressOverflow)?;
    if in_segment(image, addr, 1, true) {
        return Ok(addr);
    }
    Err(ElfError::RelocationFailed)
}

fn in_segment(image: &ElfImage, addr: u64, size: usize, executable: bool) -> bool {
    image.segments.iter().any(|segment| {
        let exec_ok = !executable || segment.is_executable();
        exec_ok
            && super::range::in_range(
                addr,
                size as u64,
                segment.vaddr.as_u64(),
                segment.size as u64,
            )
    })
}
