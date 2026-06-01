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

use alloc::string::String;

use super::{library::EmbeddedLibrary, registry::EmbeddedLibraryRegistry, version::LibraryVersion};

impl EmbeddedLibraryRegistry {
    pub fn get(&self, name: &str) -> Option<&EmbeddedLibrary> {
        self.libraries.get(name).or_else(|| self.soname_index.get(name).and_then(|library| self.libraries.get(library)))
    }

    pub fn get_by_soname(&self, soname: &str) -> Option<&EmbeddedLibrary> {
        self.soname_index.get(soname).and_then(|name| self.libraries.get(name))
    }

    pub fn contains(&self, name: &str) -> bool { self.libraries.contains_key(name) || self.soname_index.contains_key(name) }
    pub fn count(&self) -> usize { self.libraries.len() }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &EmbeddedLibrary)> { self.libraries.iter() }
    pub fn names(&self) -> impl Iterator<Item = &String> { self.libraries.keys() }
    pub fn total_size(&self) -> usize { self.libraries.values().map(|library| library.size()).sum() }
    pub fn find_compatible(&self, name: &str, required: &LibraryVersion) -> Option<&EmbeddedLibrary> { self.get(name).filter(|lib| lib.version.is_compatible(required)) }

    pub fn remove(&mut self, name: &str) -> Option<EmbeddedLibrary> {
        let library = self.libraries.remove(name)?;
        if let Some(soname) = &library.soname {
            self.soname_index.remove(soname);
        }
        Some(library)
    }
}
