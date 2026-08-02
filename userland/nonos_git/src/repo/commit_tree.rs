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

//! Recording a commit.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::commit::{encode as encode_commit, Commit, Signature};
use crate::object::ObjectKind;
use crate::odb::write_object;
use crate::oid::ObjectId;
use crate::refs::{read_head, resolve_head, update_ref, Head};
use crate::storage::Storage;

use super::error::RepoError;

/// What to record: the tree to snapshot, who is recording it and why.
pub struct CommitRequest {
    /// The tree this commit points at, already written to the database.
    pub tree: ObjectId,
    pub author: Signature,
    pub committer: Signature,
    pub message: String,
}

/// Write a commit for `request` and move the current branch to it.
///
/// The parent is whatever `HEAD` resolves to, so the first commit in a fresh
/// repository is a root commit and every later one continues the branch. The
/// commit object is written before the ref moves: if the ref write fails the
/// database holds an unreferenced object, which is harmless, whereas the other
/// order would leave a branch naming an object that is not there.
pub fn commit<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    request: &CommitRequest,
) -> Result<ObjectId, RepoError> {
    let head = read_head(storage, git_dir).ok_or(RepoError::NoHead)?;

    let mut parents = Vec::new();
    if let Some(parent) = resolve_head(storage, git_dir) {
        parents.push(parent);
    }

    let object = Commit {
        tree: request.tree,
        parents,
        author: request.author.clone(),
        committer: request.committer.clone(),
        message: request.message.clone(),
    };
    let id = write_object(storage, git_dir, ObjectKind::Commit, &encode_commit(&object))?;

    match head {
        Head::Branch(branch) => update_ref(storage, git_dir, &branch, &id)?,
        // Detached: git moves HEAD itself rather than any branch.
        Head::Detached(_) => {
            storage.write(
                &alloc::format!("{git_dir}/HEAD"),
                alloc::format!("{}\n", id.to_hex()).as_bytes(),
            )?;
        }
    }

    Ok(id)
}
