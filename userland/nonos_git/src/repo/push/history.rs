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
//! The commits a push has to carry.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::oid::ObjectId;
use crate::storage::Storage;

use super::super::error::RepoError;

/// Walk first parents and merges back from `head`, stopping at anything the
/// receiver already has, and return the commits in the order found.
///
/// A commit the other side has means every commit behind it is there too, so
/// the walk cuts the whole branch at that point rather than reading past it.
pub(super) fn commits<S: Storage>(
    storage: &S,
    git_dir: &str,
    head: &ObjectId,
    have: &[ObjectId],
) -> Result<Vec<ObjectId>, RepoError> {
    let mut found = Vec::new();
    let mut queue = Vec::new();
    queue.push(*head);

    while let Some(id) = queue.pop() {
        if have.contains(&id) || found.contains(&id) {
            continue;
        }
        let (kind, content) = read_object(storage, git_dir, &id)?;
        if kind != ObjectKind::Commit {
            return Err(RepoError::WrongKind);
        }
        let commit = crate::commit::parse(&content)?;
        found.push(id);
        queue.extend_from_slice(&commit.parents);
    }
    Ok(found)
}
