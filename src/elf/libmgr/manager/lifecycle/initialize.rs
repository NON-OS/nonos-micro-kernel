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
use crate::elf::init::{InitArrayInfo, InitArrayRunner};

use super::super::{core::LibraryManager, types::LibraryState};

impl LibraryManager {
    pub fn initialize(&mut self, id: usize) -> ElfResult<()> {
        let library = self.libraries.get_mut(&id).ok_or(ElfError::LibraryNotFound)?;
        if library.init_called {
            return Ok(());
        }
        if library.state != LibraryState::Ready {
            return Err(ElfError::InvalidState);
        }
        library.state = LibraryState::Initializing;
        if let Some(dynlink) = &library.image.dynlink_info {
            let mut runner = InitArrayRunner::new();
            if let Some(init_addr) = dynlink.init {
                runner = runner.with_init_fn(init_addr);
            }
            if let Some((addr, size)) = dynlink.init_array {
                runner = runner.with_init_array(InitArrayInfo::new(addr, size));
            }
            runner.run_all()?;
        }
        library.init_called = true;
        library.state = LibraryState::Ready;
        Ok(())
    }
}
