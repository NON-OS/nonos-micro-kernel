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

use crate::elf::errors::{ElfError, ElfResult};

use super::super::{core::LibraryManager, types::LibraryState};

impl LibraryManager {
    pub fn relocate(&mut self, id: usize) -> ElfResult<()> {
        let library = self.libraries.get_mut(&id).ok_or(ElfError::LibraryNotFound)?;
        if library.state != LibraryState::Loading { return Ok(()); }
        library.state = LibraryState::Relocating;
        if let Some(dynlink) = &library.image.dynlink_info {
            if let (Some(symtab), Some(strtab)) = (dynlink.symtab, dynlink.strtab) {
                self.symbol_resolver.parse_symbols(symtab, strtab, dynlink.strtab_size, dynlink.sym_count, library.image.base_addr, id)?;
            }
        }
        library.state = LibraryState::Ready;
        Ok(())
    }
}
