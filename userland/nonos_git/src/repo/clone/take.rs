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
//! Laying down a fetched repository.

extern crate alloc;

use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::refs::{set_head_branch, update_ref};
use crate::storage::Storage;

use super::super::checkout::checkout;
use super::super::error::RepoError;
use super::super::init::init;
use super::super::write_index::write_index;
use super::request::CloneRequest;
use super::shallow::mark_shallow;
use super::store::store_pack;

/// Initialise the repository, unpack `pack`, point the branch at the head and
/// write out the work tree and index. Returns the number of files checked out.
///
/// The index is written from the tree that was just checked out, so git sees a
/// clean work tree straight after rather than a repository where every file
/// looks deleted and untracked at once.
pub fn clone_into<S: Storage>(
    storage: &mut S,
    request: &CloneRequest<'_>,
    pack: &[u8],
) -> Result<usize, RepoError> {
    let git_dir = request.git_dir;
    init(storage, git_dir, request.branch)?;
    store_pack(storage, git_dir, pack)?;

    let (kind, content) = read_object(storage, git_dir, &request.head)?;
    if kind != ObjectKind::Commit {
        return Err(RepoError::WrongKind);
    }
    let commit = crate::commit::parse(&content)?;

    set_head_branch(storage, git_dir, request.branch)?;
    update_ref(storage, git_dir, request.branch, &request.head)?;
    if request.shallow {
        mark_shallow(storage, git_dir, &[request.head])?;
    }

    let entries = checkout(storage, git_dir, &commit.tree, request.work_tree)?;
    write_index(storage, git_dir, &entries)?;
    Ok(entries.len())
}
