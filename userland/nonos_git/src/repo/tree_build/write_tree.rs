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

//! The entry point.

use crate::index::IndexEntry;
use crate::oid::ObjectId;
use crate::storage::Storage;

use super::build::build;
use crate::repo::error::RepoError;

/// Write the tree the index describes and return its id.
pub fn write_tree<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    entries: &[IndexEntry],
) -> Result<ObjectId, RepoError> {
    build(storage, git_dir, entries, "")
}
