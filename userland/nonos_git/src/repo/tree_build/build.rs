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

//! Grouping flat index paths back into one tree per directory.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::index::IndexEntry;
use crate::object::ObjectKind;
use crate::odb::write_object;
use crate::oid::ObjectId;
use crate::repo::error::RepoError;
use crate::storage::Storage;
use crate::tree::{encode, Mode, TreeEntry};

/// Write the tree for directory `prefix`, which is empty for the root and
/// otherwise ends in a slash.
pub(super) fn build<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    entries: &[IndexEntry],
    prefix: &str,
) -> Result<ObjectId, RepoError> {
    let mut tree: Vec<TreeEntry> = Vec::new();
    let mut i = 0usize;

    while i < entries.len() {
        let rest = &entries[i].path[prefix.len()..];
        match rest.find('/') {
            None => {
                let name = String::from(rest);
                tree.push(TreeEntry { mode: entries[i].mode, name, id: entries[i].id });
                i += 1;
            }
            Some(slash) => {
                let name = &rest[..slash];
                let sub = alloc::format!("{prefix}{name}/");
                let start = i;
                while i < entries.len() && entries[i].path.starts_with(&sub) {
                    i += 1;
                }
                let id = build(storage, git_dir, &entries[start..i], &sub)?;
                tree.push(TreeEntry { mode: Mode::Directory, name: String::from(name), id });
            }
        }
    }

    let content = encode(&mut tree);
    Ok(write_object(storage, git_dir, ObjectKind::Tree, &content)?)
}
