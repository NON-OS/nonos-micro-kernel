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

//! Creating a repository.

extern crate alloc;

use alloc::format;

use crate::refs::set_head_branch;
use crate::storage::Storage;

use super::error::RepoError;

/// The layout `init` writes: the directories every later operation assumes.
const DIRS: [&str; 4] = ["objects", "refs", "refs/heads", "refs/tags"];

/// Create a repository at `git_dir`, with `HEAD` on `branch`.
///
/// This is the minimum a repository needs for git to recognise it and for the
/// operations here to work: the object and ref directories, a `HEAD` pointing
/// at an unborn branch, and a config marking it non-bare with version 0. It
/// deliberately writes no hooks, description or info files; git creates those
/// as conveniences and works without them.
pub fn init<S: Storage>(storage: &mut S, git_dir: &str, branch: &str) -> Result<(), RepoError> {
    if storage.exists(&format!("{git_dir}/HEAD")) {
        return Err(RepoError::Exists);
    }

    for dir in DIRS {
        storage.create_dir_all(&format!("{git_dir}/{dir}"))?;
    }

    // repositoryformatversion 0 is the plain sha1 format every git can read.
    // bare = false says the work tree is the directory holding this one.
    storage.write(
        &format!("{git_dir}/config"),
        b"[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n",
    )?;

    set_head_branch(storage, git_dir, branch)?;
    Ok(())
}
