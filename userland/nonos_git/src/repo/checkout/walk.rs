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
//! Walking a tree and every subtree under it.

extern crate alloc;

use alloc::vec::Vec;

use crate::index::IndexEntry;
use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::oid::ObjectId;
use crate::storage::Storage;
use crate::tree::parse;

use super::super::error::RepoError;
use super::entry::{write_entry, Written};

/// Write the tree `root` names into the work tree under `prefix`, returning
/// the index entries for the files it wrote.
///
/// Entry names are checked when the tree is parsed, which is what stops a name
/// like `..` or one holding a slash writing outside the work tree. Trees are
/// sorted with directory names ordered as though they ended in a slash, so
/// walking them depth first yields paths already in index order.
pub fn checkout<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    root: &ObjectId,
    prefix: &str,
) -> Result<Vec<IndexEntry>, RepoError> {
    let (kind, content) = read_object(storage, git_dir, root)?;
    if kind != ObjectKind::Tree {
        return Err(RepoError::WrongKind);
    }
    let entries = parse(&content)?;

    let mut staged = Vec::new();
    for entry in &entries {
        match write_entry(storage, git_dir, prefix, entry)? {
            Written::File(index_entry) => staged.push(index_entry),
            Written::Subtree(sub) => {
                storage.create_dir_all(&sub)?;
                staged.extend(checkout(storage, git_dir, &entry.id, &sub)?);
            }
            Written::Skipped => {}
        }
    }
    Ok(staged)
}
