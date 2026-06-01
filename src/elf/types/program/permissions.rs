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

use super::state::ProgramHeader;
use crate::elf::types::{phdr_flags, phdr_type};

impl ProgramHeader {
    pub fn is_load(&self) -> bool { self.p_type == phdr_type::PT_LOAD }
    pub fn is_readable(&self) -> bool { self.p_flags & phdr_flags::PF_R != 0 }
    pub fn is_writable(&self) -> bool { self.p_flags & phdr_flags::PF_W != 0 }
    pub fn is_executable(&self) -> bool { self.p_flags & phdr_flags::PF_X != 0 }
    pub fn bss_size(&self) -> u64 { self.p_memsz.saturating_sub(self.p_filesz) }

    pub fn flags_str(&self) -> &'static str {
        match (self.is_readable(), self.is_writable(), self.is_executable()) {
            (true, true, true) => "RWX",
            (true, true, false) => "RW-",
            (true, false, true) => "R-X",
            (true, false, false) => "R--",
            (false, true, true) => "-WX",
            (false, true, false) => "-W-",
            (false, false, true) => "--X",
            (false, false, false) => "---",
        }
    }
}
