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
//! Unpacking a fetched pack into the object store.

extern crate alloc;

use crate::odb::write_object;
use crate::pack::read_pack;
use crate::storage::Storage;

use super::super::error::RepoError;

/// Write every object a pack carries into `git_dir`, returning how many landed.
///
/// The reader recomputes each id from the object it reconstructed, so a pack
/// that claims one thing and delivers another is rejected before anything is
/// written.
pub fn store_pack<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    pack: &[u8],
) -> Result<usize, RepoError> {
    let objects = read_pack(pack)?;
    for object in &objects {
        write_object(storage, git_dir, object.kind, &object.data)?;
    }
    Ok(objects.len())
}
