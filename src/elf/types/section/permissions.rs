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

use super::state::SectionHeader;
use crate::elf::types::{shdr_flags, shdr_type};

impl SectionHeader {
    pub fn is_alloc(&self) -> bool { self.sh_flags & shdr_flags::SHF_ALLOC != 0 }
    pub fn is_writable(&self) -> bool { self.sh_flags & shdr_flags::SHF_WRITE != 0 }
    pub fn is_executable(&self) -> bool { self.sh_flags & shdr_flags::SHF_EXECINSTR != 0 }
    pub fn is_bss(&self) -> bool { self.sh_type == shdr_type::SHT_NOBITS }
}
