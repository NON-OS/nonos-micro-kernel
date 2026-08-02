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

//! Loading the index.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;

use crate::index::{parse, IndexEntry};
use crate::storage::{Storage, StorageError};

use super::error::RepoError;

/// Read the index, or an empty one if the repository has no index file yet,
/// which is the state between `init` and the first `add`.
pub fn read_index<S: Storage>(storage: &S, git_dir: &str) -> Result<Vec<IndexEntry>, RepoError> {
    match storage.read(&format!("{git_dir}/index")) {
        Ok(raw) => parse(&raw).map_err(RepoError::Index),
        Err(StorageError::NotFound) => Ok(Vec::new()),
        Err(e) => Err(RepoError::Storage(e)),
    }
}
