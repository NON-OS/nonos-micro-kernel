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
//! Storing a fetched pack as a pack.

extern crate alloc;

use alloc::format;

use crate::oid::ObjectId;
use crate::pack::{index_entries, read_pack, write_pack_index};
use crate::sha1::Sha1;
use crate::storage::Storage;

use super::super::error::OdbError;

/// Write `pack` and the index for it, named the way git names them.
///
/// Storing the pack whole is what makes a real repository possible at all.
/// Exploding it would mean one file per object, which for a clone of any size
/// is tens of thousands of writes and the whole tree resident at once.
pub fn store_pack_files<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    pack: &[u8],
) -> Result<usize, OdbError> {
    let objects = read_pack(pack).map_err(|_| OdbError::Malformed)?;
    let rows = index_entries(pack, &objects).ok_or(OdbError::Malformed)?;
    let body = pack.len().checked_sub(20).ok_or(OdbError::Malformed)?;
    let sha = Sha1::digest(&pack[..body]);
    let idx = write_pack_index(&rows, &sha).ok_or(OdbError::Malformed)?;

    let dir = format!("{git_dir}/objects/pack");
    storage.create_dir_all(&dir)?;
    let name = format!("{dir}/pack-{}", ObjectId::from_bytes(sha).to_hex());
    storage.write(&format!("{name}.pack"), pack)?;
    storage.write(&format!("{name}.idx"), &idx)?;
    Ok(objects.len())
}
