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

//! The order entries must appear in inside a tree.
//!
//! Git sorts by name as raw bytes, but compares a directory as though its name
//! ended in `/`. That single rule is why the file `foo.txt` sorts before the
//! directory `foo`: git compares `foo.` against `foo/`, and `.` is 0x2E while
//! `/` is 0x2F. Getting it wrong changes the tree's hash while still producing
//! a file git can parse, so it is isolated here and tested directly.

extern crate alloc;

use core::cmp::Ordering;

use super::entry::TreeEntry;

/// Compare two entries the way git orders them within a tree.
pub(super) fn compare(a: &TreeEntry, b: &TreeEntry) -> Ordering {
    let an = a.name.as_bytes();
    let bn = b.name.as_bytes();
    let mut i = 0;
    loop {
        match (byte_at(an, i, a.mode.is_dir()), byte_at(bn, i, b.mode.is_dir())) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) => match x.cmp(&y) {
                Ordering::Equal => i += 1,
                other => return other,
            },
        }
    }
}

/// The byte at `i` of a name, treating a directory as if it ended in `/`.
fn byte_at(name: &[u8], i: usize, is_dir: bool) -> Option<u8> {
    if i < name.len() {
        Some(name[i])
    } else if is_dir && i == name.len() {
        Some(b'/')
    } else {
        None
    }
}
