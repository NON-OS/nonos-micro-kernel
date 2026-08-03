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

use crate::odb::store_pack_files;
use crate::storage::Storage;

use super::super::error::RepoError;

/// Store a fetched pack, returning how many objects it carries.
///
/// The pack is kept whole with an index beside it, which is what git does and
/// what makes a repository of any size workable: exploding it would mean one
/// file per object and the whole tree resident at once.
pub fn store_pack<S: Storage>(
    storage: &mut S,
    git_dir: &str,
    pack: &[u8],
) -> Result<usize, RepoError> {
    Ok(store_pack_files(storage, git_dir, pack)?)
}
