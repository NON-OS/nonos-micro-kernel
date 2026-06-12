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
use crate::elf::loader::{ElfImage, ElfLoader};

use super::super::registry::{EmbeddedLibrary, EmbeddedLibraryRegistry};
use super::state::{EmbeddedLibraryLoader, LoadedEmbeddedLibrary};

impl<'a> EmbeddedLibraryLoader<'a> {
    pub(super) fn load_single(&mut self, library: &EmbeddedLibrary) -> ElfResult<usize> {
        let image = self.elf_loader.load_library(library.data)?;
        let load_order = self.loaded_images.len();
        self.loaded_images.push(LoadedEmbeddedLibrary {
            name: library.name.clone(),
            image,
            load_order,
        });
        Ok(load_order)
    }
}

pub fn load_embedded_library(
    registry: &EmbeddedLibraryRegistry,
    loader: &mut ElfLoader,
    name: &str,
) -> ElfResult<ElfImage> {
    let library = registry.get(name).ok_or(ElfError::LibraryNotFound)?;
    loader.load_library(library.data)
}
