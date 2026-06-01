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

use super::state::DynLinkInfo;

impl DynLinkInfo {
    pub fn add_needed(&mut self, name: String) { self.needed_libraries.push(name); }
    pub fn needs_libraries(&self) -> bool { !self.needed_libraries.is_empty() }
    pub fn library_count(&self) -> usize { self.needed_libraries.len() }
    pub fn needs_library(&self, name: &str) -> bool { self.needed_libraries.iter().any(|library| library == name) }
}
