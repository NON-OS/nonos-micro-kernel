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

use alloc::{string::String, vec::Vec};

use crate::elf::errors::{ElfError, ElfResult};

use super::state::{EmbeddedLibraryLoader, LoadedEmbeddedLibrary};

impl<'a> EmbeddedLibraryLoader<'a> {
    pub fn load(&mut self, name: &str) -> ElfResult<&LoadedEmbeddedLibrary> {
        if let Some(idx) = self.find_loaded(name) {
            return Ok(&self.loaded_images[idx]);
        }
        let library = self.registry.get(name).ok_or(ElfError::LibraryNotFound)?;
        for dep in self.registry.resolve_dependencies(library)? {
            if self.find_loaded(&dep.name).is_none() {
                self.load_single(dep)?;
            }
        }
        let idx = self.load_single(library)?;
        Ok(&self.loaded_images[idx])
    }

    pub fn load_all_dependencies(&mut self, library_name: &str) -> ElfResult<Vec<String>> {
        let library = self.registry.get(library_name).ok_or(ElfError::LibraryNotFound)?;
        let mut loaded_names = Vec::new();
        for dep in self.registry.resolve_dependencies(library)? {
            if self.find_loaded(&dep.name).is_none() {
                self.load_single(dep)?;
                loaded_names.push(dep.name.clone());
            }
        }
        Ok(loaded_names)
    }
}
