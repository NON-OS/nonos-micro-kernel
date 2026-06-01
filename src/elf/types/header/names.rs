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

use super::state::ElfHeader;
use crate::elf::types::{elf_type, machine};

impl ElfHeader {
    pub fn type_name(&self) -> &'static str {
        match self.e_type {
            elf_type::ET_NONE => "NONE",
            elf_type::ET_REL => "REL",
            elf_type::ET_EXEC => "EXEC",
            elf_type::ET_DYN => "DYN",
            elf_type::ET_CORE => "CORE",
            _ => "UNKNOWN",
        }
    }

    pub fn machine_name(&self) -> &'static str {
        match self.e_machine {
            machine::EM_NONE => "None",
            machine::EM_386 => "Intel 80386",
            machine::EM_X86_64 => "AMD x86-64",
            machine::EM_AARCH64 => "AArch64",
            machine::EM_RISCV => "RISC-V",
            _ => "Unknown",
        }
    }
}
