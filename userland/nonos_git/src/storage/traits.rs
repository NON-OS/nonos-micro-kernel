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

//! The file operations a repository needs.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::error::StorageError;

/// What a repository asks of the filesystem underneath it.
pub trait Storage {
    /// Read a whole file.
    fn read(&self, path: &str) -> Result<Vec<u8>, StorageError>;

    /// Write a whole file, creating it and any missing parent directories,
    /// replacing any previous contents.
    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), StorageError>;

    /// Whether a path exists, as a file or a directory.
    fn exists(&self, path: &str) -> bool;

    /// Create a directory and any missing parents. Succeeds if it is already
    /// there, so callers do not have to check first.
    fn create_dir_all(&mut self, path: &str) -> Result<(), StorageError>;

    /// The names directly inside a directory, without `.` or `..`. The order is
    /// not specified; callers that need one sort for themselves.
    fn read_dir(&self, path: &str) -> Result<Vec<String>, StorageError>;

    /// Whether a path is a directory. False for a file or a missing path.
    fn is_dir(&self, path: &str) -> bool;
}
