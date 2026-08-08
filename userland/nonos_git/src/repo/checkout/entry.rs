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
//! One entry of a tree being written out.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use crate::index::IndexEntry;
use crate::object::ObjectKind;
use crate::odb::read_object;
use crate::storage::Storage;
use crate::tree::{Mode, TreeEntry};

use super::super::error::RepoError;

/// What a tree entry turned into.
pub enum Written {
    /// A file was written and should be recorded in the index.
    File(IndexEntry),
    /// A subtree to descend into, with the prefix its children sit under.
    Subtree(String),
    /// A submodule: its commit lives in another repository, so there is
    /// nothing here to write.
    Skipped,
}

/// Write one entry of a tree into the work tree under `prefix`.
pub(super) fn write_entry<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    prefix: &str,
    entry: &TreeEntry,
) -> Result<Written, RepoError> {
    let path = format!("{prefix}{}", entry.name);
    match entry.mode {
        Mode::Directory => return Ok(Written::Subtree(format!("{path}/"))),
        Mode::Submodule => return Ok(Written::Skipped),
        _ => {}
    }
    let (kind, content) = read_object(storage, git_dir, &entry.id)?;
    if kind != ObjectKind::Blob {
        return Err(RepoError::WrongKind);
    }
    storage.write(&path, &content)?;
    Ok(Written::File(IndexEntry {
        path,
        mode: entry.mode,
        id: entry.id,
        size: content.len().min(u32::MAX as usize) as u32,
    }))
}
