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

use crate::elf::loader::{ElfImage, ElfLoader};

use super::super::registry::EmbeddedLibraryRegistry;

pub struct EmbeddedLibraryLoader<'a> {
    pub(super) registry: &'a EmbeddedLibraryRegistry,
    pub(super) elf_loader: &'a mut ElfLoader,
    pub(super) loaded_images: Vec<LoadedEmbeddedLibrary>,
}

#[derive(Debug)]
pub struct LoadedEmbeddedLibrary {
    pub name: String,
    pub image: ElfImage,
    pub load_order: usize,
}

impl<'a> EmbeddedLibraryLoader<'a> {
    pub fn new(registry: &'a EmbeddedLibraryRegistry, elf_loader: &'a mut ElfLoader) -> Self {
        Self { registry, elf_loader, loaded_images: Vec::new() }
    }

    pub(super) fn find_loaded(&self, name: &str) -> Option<usize> {
        self.loaded_images.iter().position(|library| library.name == name)
    }
}
