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

//! Storing an object.

use crate::object::{frame, ObjectKind};
use crate::oid::ObjectId;
use crate::storage::Storage;
use crate::zlib::compress;

use super::error::OdbError;
use super::object_dir::object_dir;
use super::object_path::object_path;

/// Frame `content` as `kind`, compress it and store it under its own id,
/// returning that id.
///
/// An object already present is left alone rather than rewritten: the id is the
/// hash of the content, so a file already at that path holds these exact bytes,
/// and skipping the write keeps adding an unchanged file cheap.
pub fn write_object<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    kind: ObjectKind,
    content: &[u8],
) -> Result<ObjectId, OdbError> {
    let (framed, id) = frame(kind, content);
    let path = object_path(git_dir, &id);
    if storage.exists(&path) {
        return Ok(id);
    }
    storage.create_dir_all(&object_dir(git_dir, &id))?;
    storage.write(&path, &compress(&framed))?;
    Ok(id)
}
