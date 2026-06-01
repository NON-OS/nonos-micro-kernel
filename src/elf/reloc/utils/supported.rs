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

use crate::elf::types::reloc_type;

pub fn is_supported(reloc_type: u32) -> bool {
    matches!(
        reloc_type,
        reloc_type::R_X86_64_NONE
            | reloc_type::R_X86_64_64
            | reloc_type::R_X86_64_PC32
            | reloc_type::R_X86_64_GOT32
            | reloc_type::R_X86_64_PLT32
            | reloc_type::R_X86_64_GLOB_DAT
            | reloc_type::R_X86_64_JUMP_SLOT
            | reloc_type::R_X86_64_RELATIVE
            | reloc_type::R_X86_64_GOTPCREL
            | reloc_type::R_X86_64_32
            | reloc_type::R_X86_64_32S
            | reloc_type::R_X86_64_16
            | reloc_type::R_X86_64_PC16
            | reloc_type::R_X86_64_8
            | reloc_type::R_X86_64_PC8
            | reloc_type::R_X86_64_IRELATIVE
    )
}
