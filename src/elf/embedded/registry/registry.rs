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

extern crate alloc;

use alloc::{collections::BTreeMap, string::String};

use super::library::EmbeddedLibrary;
use crate::elf::errors::{ElfError, ElfResult};

pub struct EmbeddedLibraryRegistry {
    pub(super) libraries: BTreeMap<String, EmbeddedLibrary>,
    pub(super) soname_index: BTreeMap<String, String>,
}

impl EmbeddedLibraryRegistry {
    pub fn new() -> Self { Self { libraries: BTreeMap::new(), soname_index: BTreeMap::new() } }

    pub fn register(&mut self, library: EmbeddedLibrary) -> ElfResult<()> {
        if self.libraries.contains_key(&library.name) {
            return Err(ElfError::LibraryAlreadyLoaded);
        }
        if let Some(soname) = &library.soname {
            self.soname_index.insert(soname.clone(), library.name.clone());
        }
        self.libraries.insert(library.name.clone(), library);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.libraries.clear();
        self.soname_index.clear();
    }
}

impl Default for EmbeddedLibraryRegistry {
    fn default() -> Self { Self::new() }
}
