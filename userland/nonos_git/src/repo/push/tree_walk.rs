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
//! Everything reachable from one tree.

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::oid::ObjectId;
use crate::storage::Storage;
use crate::tree::{parse, Mode};

use super::super::error::RepoError;

/// Add the tree `root` and everything under it to `out`, skipping ids already
/// in `seen`. Submodules are commits in another repository, so they are not
/// followed.
pub(super) fn collect<S: Storage>(
    storage: &S,
    git_dir: &str,
    root: &ObjectId,
    seen: &mut BTreeSet<ObjectId>,
    out: &mut Vec<(ObjectKind, Vec<u8>)>,
) -> Result<(), RepoError> {
    if !seen.insert(*root) {
        return Ok(());
    }
    let (kind, content) = read_object(storage, git_dir, root)?;
    if kind != ObjectKind::Tree {
        return Err(RepoError::WrongKind);
    }
    let entries = parse(&content)?;
    out.push((ObjectKind::Tree, content));

    for entry in &entries {
        match entry.mode {
            Mode::Directory => collect(storage, git_dir, &entry.id, seen, out)?,
            Mode::Submodule => {}
            _ => {
                if seen.insert(entry.id) {
                    let (blob, data) = read_object(storage, git_dir, &entry.id)?;
                    if blob != ObjectKind::Blob {
                        return Err(RepoError::WrongKind);
                    }
                    out.push((ObjectKind::Blob, data));
                }
            }
        }
    }
    Ok(())
}
