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

use super::{aux_type, entry::AuxEntry};

impl AuxEntry {
    pub fn type_name(&self) -> &'static str {
        match self.a_type {
            aux_type::AT_NULL => "AT_NULL",
            aux_type::AT_IGNORE => "AT_IGNORE",
            aux_type::AT_EXECFD => "AT_EXECFD",
            aux_type::AT_PHDR => "AT_PHDR",
            aux_type::AT_PHENT => "AT_PHENT",
            aux_type::AT_PHNUM => "AT_PHNUM",
            aux_type::AT_PAGESZ => "AT_PAGESZ",
            aux_type::AT_BASE => "AT_BASE",
            aux_type::AT_FLAGS => "AT_FLAGS",
            aux_type::AT_ENTRY => "AT_ENTRY",
            aux_type::AT_UID => "AT_UID",
            aux_type::AT_EUID => "AT_EUID",
            aux_type::AT_GID => "AT_GID",
            aux_type::AT_EGID => "AT_EGID",
            aux_type::AT_PLATFORM => "AT_PLATFORM",
            aux_type::AT_HWCAP => "AT_HWCAP",
            aux_type::AT_CLKTCK => "AT_CLKTCK",
            aux_type::AT_SECURE => "AT_SECURE",
            aux_type::AT_RANDOM => "AT_RANDOM",
            aux_type::AT_HWCAP2 => "AT_HWCAP2",
            aux_type::AT_EXECFN => "AT_EXECFN",
            aux_type::AT_SYSINFO_EHDR => "AT_SYSINFO_EHDR",
            _ => "AT_UNKNOWN",
        }
    }
}
