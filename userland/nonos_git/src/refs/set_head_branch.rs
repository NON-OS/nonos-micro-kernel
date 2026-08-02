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

//! Pointing `HEAD` at a branch.

extern crate alloc;

use alloc::format;

use crate::storage::{Storage, StorageError};

use super::name::is_valid_ref_name;

/// Point `HEAD` at a branch, whether or not that branch exists yet. An unborn
/// branch is how a repository sits between `init` and its first commit.
pub fn set_head_branch<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    branch: &str,
) -> Result<(), StorageError> {
    if !is_valid_ref_name(branch) {
        return Err(StorageError::Io);
    }
    storage.write(&format!("{git_dir}/HEAD"), format!("ref: refs/heads/{branch}\n").as_bytes())
}
