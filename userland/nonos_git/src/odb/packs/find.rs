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
//! Reading an object out of the packs.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;

use crate::object::ObjectKind;
use crate::oid::ObjectId;
use crate::pack::{pack_lookup, read_at};
use crate::storage::Storage;

use super::names::pack_names;

/// Find `id` in any pack, or None if no pack holds it.
///
/// The index is consulted first, so a pack that does not hold the object
/// costs one index read rather than a walk of the whole file.
pub fn read_from_packs<S: Storage>(
    storage: &S,
    git_dir: &str,
    id: &ObjectId,
) -> Option<(ObjectKind, Vec<u8>)> {
    for base in pack_names(storage, git_dir) {
        let Ok(idx) = storage.read(&format!("{base}.idx")) else {
            continue;
        };
        let Some(offset) = pack_lookup(&idx, id) else {
            continue;
        };
        let Ok(pack) = storage.read(&format!("{base}.pack")) else {
            continue;
        };
        // A reference delta names its base by id, which the same index
        // resolves back to an offset in this pack.
        let find = |want: &ObjectId| pack_lookup(&idx, want).map(|at| at as usize);
        if let Ok((kind, data)) = read_at(&pack, offset as usize, &find, 0) {
            return Some((kind, data));
        }
    }
    None
}
