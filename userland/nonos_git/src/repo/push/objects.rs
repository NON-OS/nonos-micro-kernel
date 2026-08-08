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
//! The object list a push sends.

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::oid::ObjectId;
use crate::storage::Storage;

use super::super::error::RepoError;
use super::history::commits;
use super::tree_walk::collect;

/// Every object the receiver needs to hold `head`, given the ids in `have`
/// that it already has.
///
/// Commits come before the trees and blobs they name. Nothing in the format
/// requires that order, but it keeps a pack readable in one forward pass.
pub fn objects_to_send<S: Storage>(
    storage: &S,
    git_dir: &str,
    head: &ObjectId,
    have: &[ObjectId],
) -> Result<Vec<(ObjectKind, Vec<u8>)>, RepoError> {
    let chain = commits(storage, git_dir, head, have)?;
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();

    for id in &chain {
        let (_kind, content) = read_object(storage, git_dir, id)?;
        let commit = crate::commit::parse(&content)?;
        out.push((ObjectKind::Commit, content));
        collect(storage, git_dir, &commit.tree, &mut seen, &mut out)?;
    }
    Ok(out)
}
