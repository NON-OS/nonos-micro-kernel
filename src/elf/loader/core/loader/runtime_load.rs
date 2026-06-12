// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::state::ElfLoader;
use crate::elf::errors::ElfError;
use crate::elf::loader::ElfImage;

impl ElfLoader {
    pub fn load_executable(&mut self, elf_data: &[u8]) -> Result<ElfImage, ElfError> {
        let active =
            crate::memory::paging::manager::active_asid().ok_or(ElfError::NotInitialized)?;
        self.load_executable_into(elf_data, active)
    }

    pub fn load_library(&mut self, elf_data: &[u8]) -> Result<ElfImage, ElfError> {
        self.load_executable(elf_data)
    }
}
