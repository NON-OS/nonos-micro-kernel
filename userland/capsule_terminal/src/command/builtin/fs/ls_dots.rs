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

//! The `.` and `..` rows `-a` adds. They are real navigable paths, not
//! invented entries, but the vfs has no stat for either, so they carry only
//! what is known for certain: they are directories.

use alloc::vec::Vec;

use super::ls_long::Row;

/// `..` is omitted at the root, where it would name the root again.
pub fn dot_names(base: &[u8], all: bool) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    if !all {
        return out;
    }
    out.push(Vec::from(&b"./"[..]));
    if base != b"/" {
        out.push(Vec::from(&b"../"[..]));
    }
    out
}

pub fn dot_rows(names: &[Vec<u8>]) -> Vec<Row> {
    names
        .iter()
        .map(|n| Row { name: n.clone(), size: 0, is_dir: true, mtime: 0, writable: false })
        .collect()
}
