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

//! Pointing a branch at a commit.

extern crate alloc;

use alloc::format;

use crate::oid::ObjectId;
use crate::storage::{Storage, StorageError};

use super::name::is_valid_ref_name;

/// Point `refs/heads/<branch>` at `id`.
///
/// The name is validated before it is joined into a path, so a branch cannot
/// be made to write outside `refs/heads`. Git stores the id as hex with a
/// trailing newline, and other tools expect exactly that.
pub fn update_ref<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    branch: &str,
    id: &ObjectId,
) -> Result<(), StorageError> {
    if !is_valid_ref_name(branch) {
        return Err(StorageError::Io);
    }
    let path = format!("{git_dir}/refs/heads/{branch}");
    storage.write(&path, format!("{}\n", id.to_hex()).as_bytes())
}
