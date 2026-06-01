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

use super::{library::EmbeddedLibrary, registry::EmbeddedLibraryRegistry};
use crate::elf::errors::{ElfError, ElfResult};

impl EmbeddedLibraryRegistry {
    pub fn resolve_dependencies<'a>(&'a self, library: &'a EmbeddedLibrary) -> ElfResult<Vec<&'a EmbeddedLibrary>> {
        let mut resolved = Vec::new();
        let mut visited = Vec::new();
        self.resolve_deps_recursive(library, &mut resolved, &mut visited)?;
        Ok(resolved)
    }

    fn resolve_deps_recursive<'a>(&'a self, library: &'a EmbeddedLibrary, resolved: &mut Vec<&'a EmbeddedLibrary>, visited: &mut Vec<String>) -> ElfResult<()> {
        if visited.contains(&library.name) {
            return Err(ElfError::CircularDependency);
        }
        visited.push(library.name.clone());
        for dep_name in &library.dependencies {
            let dep = self.get(dep_name).ok_or(ElfError::LibraryNotFound)?;
            if !resolved.iter().any(|entry| entry.name == dep.name) {
                self.resolve_deps_recursive(dep, resolved, visited)?;
            }
        }
        if !resolved.iter().any(|entry| entry.name == library.name) {
            resolved.push(library);
        }
        visited.pop();
        Ok(())
    }
}
