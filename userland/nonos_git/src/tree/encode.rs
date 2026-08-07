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

//! Writing a tree's content bytes.

extern crate alloc;

use alloc::vec::Vec;

use super::entry::TreeEntry;
use super::sort::sort;

/// Encode `entries` as a tree's content: for each entry, the octal mode, a
/// space, the name, a NUL, then the object id as twenty raw bytes, not hex.
///
/// The entries are sorted into git's order first, so a caller may pass them in
/// any order and still get the id git would produce for that directory.
pub fn encode(entries: &mut [TreeEntry]) -> Vec<u8> {
    sort(entries);
    let mut out = Vec::new();
    for e in entries.iter() {
        out.extend_from_slice(e.mode.as_bytes());
        out.push(b' ');
        out.extend_from_slice(e.name.as_bytes());
        out.push(0);
        out.extend_from_slice(e.id.as_bytes());
    }
    out
}
