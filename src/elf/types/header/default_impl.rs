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
use crate::elf::types::{ProgramHeader, SectionHeader};

impl Default for ElfHeader {
    fn default() -> Self {
        Self {
            ident: [0; 16],
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: ElfHeader::SIZE as u16,
            e_phentsize: ProgramHeader::SIZE as u16,
            e_phnum: 0,
            e_shentsize: SectionHeader::SIZE as u16,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}
