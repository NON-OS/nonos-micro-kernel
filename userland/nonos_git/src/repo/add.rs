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

//! Staging a file.

extern crate alloc;

use alloc::string::String;

use crate::index::{stage, IndexEntry};
use crate::object::ObjectKind;
use crate::odb::write_object;
use crate::oid::ObjectId;
use crate::storage::Storage;
use crate::tree::Mode;

use super::error::RepoError;
use super::read_index::read_index;
use super::write_index::write_index;

/// Stage `path` from the work tree: store its contents as a blob and record it
/// in the index under that path.
///
/// `mode` is the caller's, since the filesystem underneath does not carry an
/// executable bit the same way everywhere; the terminal passes `Mode::File`
/// unless it knows otherwise. Staging the same path again replaces its entry,
/// so the index holds one row per path.
pub fn add<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    work_path: &str,
    mode: Mode,
) -> Result<ObjectId, RepoError> {
    let content = storage.read(work_path)?;
    let id = write_object(storage, git_dir, ObjectKind::Blob, &content)?;

    let mut entries = read_index(storage, git_dir)?;
    stage(
        &mut entries,
        IndexEntry {
            path: String::from(work_path),
            mode,
            id,
            // The index carries the staged size; a file larger than this field
            // still stages correctly, since the size is a cache and the blob id
            // is what the commit is built from.
            size: content.len().min(u32::MAX as usize) as u32,
        },
    );
    write_index(storage, git_dir, &entries)?;
    Ok(id)
}
