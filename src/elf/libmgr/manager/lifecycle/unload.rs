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

use super::super::core::LibraryManager;

impl LibraryManager {
    pub fn unload(&mut self, id: usize) -> ElfResult<()> {
        let should_unload = self.libraries.get_mut(&id).ok_or(ElfError::LibraryNotFound)?.release();
        if should_unload {
            self.finalize(id)?;
            if let Some(library) = self.libraries.remove(&id) {
                self.name_index.remove(&library.name);
                if let Some(soname) = &library.soname {
                    self.soname_index.remove(soname);
                }
                self.addr_index.remove(&library.base_addr().as_u64());
                self.load_order.retain(|&entry| entry != id);
            }
        }
        Ok(())
    }
}
