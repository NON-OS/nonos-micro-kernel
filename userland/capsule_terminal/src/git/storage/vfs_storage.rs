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

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_git::{Storage, StorageError};

use super::join::join;

/// Carries the owning pid every VFS call needs, and the work tree the
/// repository paths are relative to.
pub struct VfsStorage {
    pub(super) owner: u32,
    root: String,
}

impl VfsStorage {
    pub fn new(owner: u32, root: &str) -> VfsStorage {
        VfsStorage { owner, root: String::from(root) }
    }

    /// The work tree, as the shell's working directory gave it.
    pub fn root(&self) -> &str {
        &self.root
    }

    pub(super) fn full(&self, path: &str) -> String {
        join(&self.root, path)
    }
}

impl Storage for VfsStorage {
    fn read(&self, path: &str) -> Result<Vec<u8>, StorageError> {
        super::files::read(self, path)
    }

    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), StorageError> {
        super::files::write(self, path, data)
    }

    fn exists(&self, path: &str) -> bool {
        super::files::exists(self, path)
    }

    fn create_dir_all(&mut self, path: &str) -> Result<(), StorageError> {
        super::dirs::create_dir_all(self, path)
    }

    fn read_dir(&self, path: &str) -> Result<Vec<String>, StorageError> {
        super::dirs::read_dir(self, path)
    }

    fn is_dir(&self, path: &str) -> bool {
        super::files::is_dir(self, path)
    }
}
