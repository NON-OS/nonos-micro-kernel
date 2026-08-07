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
//! File operations over the VFS client.

extern crate alloc;

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_git::StorageError;

use super::vfs_storage::VfsStorage;

/// Bound on a single read. A git object or index far exceeds anything a
/// terminal session writes, and a bound keeps one call from being asked for
/// unbounded memory.
const MAX_READ: u32 = 8 * 1024 * 1024;

pub(super) fn read(s: &VfsStorage, path: &str) -> Result<Vec<u8>, StorageError> {
    let full = s.full(path);
    vfs::read_file(s.owner, full.as_bytes(), MAX_READ).map_err(|_| StorageError::NotFound)
}

pub(super) fn write(s: &VfsStorage, path: &str, data: &[u8]) -> Result<(), StorageError> {
    let full = s.full(path);
    if let Some(parent) = full.rfind('/') {
        // The VFS creates no parents, so a nested object path needs them made
        // first; an existing directory is not an error.
        let _ = super::dirs::mkdir_parents(s, &full[..parent]);
    }
    vfs::write_file(s.owner, full.as_bytes(), data).map_err(|_| StorageError::Io)
}

pub(super) fn exists(s: &VfsStorage, path: &str) -> bool {
    let full = s.full(path);
    vfs::stat_full(s.owner, full.as_bytes()).is_ok()
}

pub(super) fn is_dir(s: &VfsStorage, path: &str) -> bool {
    let full = s.full(path);
    matches!(vfs::stat_full(s.owner, full.as_bytes()), Ok((_, true, _, _)))
}
