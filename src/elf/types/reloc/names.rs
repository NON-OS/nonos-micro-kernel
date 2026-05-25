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

use super::state::RelaEntry;
use crate::elf::types::reloc_type;

impl RelaEntry {
    pub fn type_name(&self) -> &'static str {
        match self.reloc_type() {
            reloc_type::R_X86_64_NONE => "R_X86_64_NONE",
            reloc_type::R_X86_64_64 => "R_X86_64_64",
            reloc_type::R_X86_64_PC32 => "R_X86_64_PC32",
            reloc_type::R_X86_64_GOT32 => "R_X86_64_GOT32",
            reloc_type::R_X86_64_PLT32 => "R_X86_64_PLT32",
            reloc_type::R_X86_64_COPY => "R_X86_64_COPY",
            reloc_type::R_X86_64_GLOB_DAT => "R_X86_64_GLOB_DAT",
            reloc_type::R_X86_64_JUMP_SLOT => "R_X86_64_JUMP_SLOT",
            reloc_type::R_X86_64_RELATIVE => "R_X86_64_RELATIVE",
            reloc_type::R_X86_64_GOTPCREL => "R_X86_64_GOTPCREL",
            reloc_type::R_X86_64_32 => "R_X86_64_32",
            reloc_type::R_X86_64_32S => "R_X86_64_32S",
            _ => "UNKNOWN",
        }
    }
}
