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

//! Putting one path into the index.

extern crate alloc;

use alloc::vec::Vec;

use super::entry::IndexEntry;

/// Insert or replace `entry`, keeping the index sorted by path.
///
/// Git requires the entries to be sorted so it can binary search them, and
/// staging the same path twice must replace rather than duplicate it, or the
/// tree built from the index would hold the path twice.
pub fn stage(entries: &mut Vec<IndexEntry>, entry: IndexEntry) {
    match entries.binary_search_by(|e| e.path.as_str().cmp(entry.path.as_str())) {
        Ok(at) => entries[at] = entry,
        Err(at) => entries.insert(at, entry),
    }
}
