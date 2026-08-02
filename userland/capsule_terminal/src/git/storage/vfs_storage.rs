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
//! The adapter itself.

use nonos_git::Storage;

/// Carries the owning pid every VFS call needs, and the work tree the
/// repository paths are relative to.
pub struct VfsStorage {
    pub(super) owner: u32,
    pub(super) root: alloc::string::String,
}

impl VfsStorage {
    pub fn new(owner: u32, root: &str) -> VfsStorage {
        VfsStorage { owner, root: alloc::string::String::from(root) }
    }

    /// A repository-relative path joined onto the work tree.
    pub(super) fn full(&self, path: &str) -> alloc::string::String {
        let mut out = self.root.clone();
        if !out.ends_with('/') {
            out.push('/');
        }
        out.push_str(path);
        out
    }
}

impl Storage for VfsStorage {
    fn read(&self, path: &str) -> Result<alloc::vec::Vec<u8>, nonos_git::StorageError> {
        super::files::read(self, path)
    }

    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), nonos_git::StorageError> {
        super::files::write(self, path, data)
    }

    fn exists(&self, path: &str) -> bool {
        super::files::exists(self, path)
    }

    fn create_dir_all(&mut self, path: &str) -> Result<(), nonos_git::StorageError> {
        super::dirs::create_dir_all(self, path)
    }

    fn read_dir(&self, path: &str) -> Result<alloc::vec::Vec<alloc::string::String>, nonos_git::StorageError> {
        super::dirs::read_dir(self, path)
    }

    fn is_dir(&self, path: &str) -> bool {
        super::files::is_dir(self, path)
    }
}
