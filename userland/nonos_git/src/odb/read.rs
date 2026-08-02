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

//! Loading an object.

extern crate alloc;

use alloc::vec::Vec;

use crate::object::{unframe, ObjectKind};
use crate::oid::ObjectId;
use crate::sha1::Sha1;
use crate::storage::Storage;
use crate::zlib::decompress;

use super::error::OdbError;
use super::object_path::object_path;

/// Load the object named by `id`, returning its kind and content.
///
/// The content is hashed again and checked against the id it was stored under.
/// That is what makes a silently damaged object database an error rather than
/// wrong data handed to a caller that has no way to tell.
pub fn read_object<S: Storage>(
    storage: &S,
    git_dir: &str,
    id: &ObjectId,
) -> Result<(ObjectKind, Vec<u8>), OdbError> {
    let path = object_path(git_dir, id);
    let stored = storage.read(&path)?;
    let framed = decompress(&stored).map_err(OdbError::Corrupt)?;

    if ObjectId::from_bytes(Sha1::digest(&framed)) != *id {
        return Err(OdbError::IdMismatch);
    }

    let (kind, content) = unframe(&framed).ok_or(OdbError::Malformed)?;
    Ok((kind, content.to_vec()))
}
