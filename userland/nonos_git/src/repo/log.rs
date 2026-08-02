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

//! Walking history back from `HEAD`.

extern crate alloc;

use alloc::vec::Vec;

use crate::commit::{parse as parse_commit, Commit};
use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::oid::ObjectId;
use crate::refs::resolve_head;
use crate::storage::Storage;

use super::error::RepoError;

/// One entry in the history: a commit and the id it is stored under.
pub struct LogEntry {
    pub id: ObjectId,
    pub commit: Commit,
}

/// The commits reachable from `HEAD` by first parent, newest first, at most
/// `limit` of them.
///
/// Following only the first parent is what makes this a branch's history rather
/// than every ancestor: a merge's other parents are reachable but belong to the
/// branches that were merged in. The walk stops at a root commit, and `limit`
/// bounds it so a caller cannot be made to read an unbounded history.
pub fn log<S: Storage>(
    storage: &S,
    git_dir: &str,
    limit: usize,
) -> Result<Vec<LogEntry>, RepoError> {
    let mut out = Vec::new();
    let mut next = resolve_head(storage, git_dir);

    while let Some(id) = next {
        if out.len() >= limit {
            break;
        }
        let (kind, content) = read_object(storage, git_dir, &id)?;
        if kind != ObjectKind::Commit {
            return Err(RepoError::WrongKind);
        }
        let commit = parse_commit(&content)?;
        next = commit.parents.first().copied();
        out.push(LogEntry { id, commit });
    }

    Ok(out)
}
