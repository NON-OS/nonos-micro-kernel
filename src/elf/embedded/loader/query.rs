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

use super::state::{EmbeddedLibraryLoader, LoadedEmbeddedLibrary};

impl<'a> EmbeddedLibraryLoader<'a> {
    pub fn get_loaded(&self, name: &str) -> Option<&LoadedEmbeddedLibrary> { self.find_loaded(name).map(|idx| &self.loaded_images[idx]) }
    pub fn loaded_count(&self) -> usize { self.loaded_images.len() }
    pub fn loaded_libraries(&self) -> &[LoadedEmbeddedLibrary] { &self.loaded_images }

    pub fn unload(&mut self, name: &str) -> ElfResult<()> {
        let idx = self.find_loaded(name).ok_or(ElfError::LibraryNotFound)?;
        self.loaded_images.remove(idx);
        Ok(())
    }

    pub fn unload_all(&mut self) {
        self.loaded_images.clear();
    }
}
