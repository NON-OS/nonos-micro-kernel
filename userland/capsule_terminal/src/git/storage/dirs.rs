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
//! Directory operations over the VFS client.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_git::StorageError;

use super::vfs_storage::VfsStorage;

pub(super) fn create_dir_all(s: &VfsStorage, path: &str) -> Result<(), StorageError> {
    let full = s.full(path);
    mkdir_parents(s, &full)
}

/// Make every component of an absolute path. The VFS creates one level at a
/// time and reports an existing directory as an error, which is ignored here
/// so the call is idempotent.
pub(super) fn mkdir_parents(s: &VfsStorage, full: &str) -> Result<(), StorageError> {
    let mut at = String::new();
    for part in full.split('/').filter(|p| !p.is_empty()) {
        at.push('/');
        at.push_str(part);
        let _ = vfs::mkdir(s.owner, at.as_bytes());
    }
    match vfs::stat_full(s.owner, full.as_bytes()) {
        Ok((_, true, _, _)) => Ok(()),
        _ => Err(StorageError::Io),
    }
}

pub(super) fn read_dir(s: &VfsStorage, path: &str) -> Result<Vec<String>, StorageError> {
    let full = s.full(path);
    let paths = vfs::list_paths(s.owner, full.as_bytes()).map_err(|_| StorageError::NotFound)?;
    // The server returns full paths; the trait wants the names inside.
    Ok(paths.iter().filter_map(|p| p.rsplit('/').next().map(String::from)).collect())
}
