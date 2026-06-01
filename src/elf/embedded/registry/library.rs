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

use super::version::LibraryVersion;

#[derive(Debug, Clone)]
pub struct EmbeddedLibrary {
    pub name: String,
    pub soname: Option<String>,
    pub data: &'static [u8],
    pub version: LibraryVersion,
    pub dependencies: Vec<String>,
}

impl EmbeddedLibrary {
    pub fn new(name: impl Into<String>, data: &'static [u8]) -> Self {
        Self { name: name.into(), soname: None, data, version: LibraryVersion::new(0, 0, 0), dependencies: Vec::new() }
    }

    pub fn with_name(data: &'static [u8], name: String) -> Self { Self::new(name, data) }
    pub fn with_soname(mut self, soname: String) -> Self { self.soname = Some(soname); self }
    pub fn with_version(mut self, version: LibraryVersion) -> Self { self.version = version; self }
    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self { self.dependencies = deps; self }
    pub fn size(&self) -> usize { self.data.len() }
    pub fn as_ptr(&self) -> *const u8 { self.data.as_ptr() }
}
