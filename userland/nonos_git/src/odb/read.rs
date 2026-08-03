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

use crate::object::{frame, unframe, ObjectKind};
use crate::oid::ObjectId;
use crate::sha1::Sha1;
use crate::storage::Storage;
use crate::zlib::decompress;

use super::error::OdbError;
use super::object_path::object_path;
use super::packs::read_from_packs;

/// Load the object named by `id`, returning its kind and content.
///
/// Loose first, then the packs. A fetched pack is stored whole rather than
/// exploded, so most of a cloned repository lives in one and this is the only
/// path that reaches it.
///
/// Either way the content is hashed again and checked against the id it was
/// asked for. That is what makes a silently damaged object database an error
/// rather than wrong data handed to a caller with no way to tell.
pub fn read_object<S: Storage>(
    storage: &S,
    git_dir: &str,
    id: &ObjectId,
) -> Result<(ObjectKind, Vec<u8>), OdbError> {
    match storage.read(&object_path(git_dir, id)) {
        Ok(stored) => loose(&stored, id),
        Err(_) => packed(storage, git_dir, id),
    }
}

fn loose(stored: &[u8], id: &ObjectId) -> Result<(ObjectKind, Vec<u8>), OdbError> {
    let framed = decompress(stored).map_err(OdbError::Corrupt)?;
    if ObjectId::from_bytes(Sha1::digest(&framed)) != *id {
        return Err(OdbError::IdMismatch);
    }
    let (kind, content) = unframe(&framed).ok_or(OdbError::Malformed)?;
    Ok((kind, content.to_vec()))
}

fn packed<S: Storage>(
    storage: &S,
    git_dir: &str,
    id: &ObjectId,
) -> Result<(ObjectKind, Vec<u8>), OdbError> {
    let (kind, content) = read_from_packs(storage, git_dir, id).ok_or(OdbError::NotFound)?;
    // The pack index says this offset holds this id. Framing the result and
    // hashing it is what checks that claim rather than trusting it.
    let (_framed, actual) = frame(kind, &content);
    if actual != *id {
        return Err(OdbError::IdMismatch);
    }
    Ok((kind, content))
}
