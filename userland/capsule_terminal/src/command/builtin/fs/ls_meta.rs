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

//! Per-entry metadata for the flags that need it, and the sort orders built on
//! it. `stat_full` is the only source of size, mtime and the writable bit.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::ls_flags::LsFlags;
use super::ls_long::Row;
use super::pid;
use crate::term::state::State;

pub fn meta(state: &mut State, base: &[u8], names: &[Vec<u8>], flags: &LsFlags) -> Vec<Row> {
    let owner = pid(state);
    let mut rows: Vec<Row> = Vec::new();
    for name in names {
        let trimmed = name.strip_suffix(b"/").unwrap_or(name);
        let mut full = base.to_vec();
        full.extend_from_slice(trimmed);
        let fallback = (0u64, name.last() == Some(&b'/'), 0u64, false);
        let m = vfs::stat_full(owner, &full).unwrap_or(fallback);
        rows.push(Row { name: name.clone(), size: m.0, is_dir: m.1, mtime: m.2, writable: m.3 });
    }
    if flags.by_time {
        rows.sort_by(|a, b| b.mtime.cmp(&a.mtime));
    } else if flags.by_size {
        rows.sort_by(|a, b| b.size.cmp(&a.size));
    }
    rows
}

pub fn subdirs(base: &[u8], names: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for name in names {
        if name.last() == Some(&b'/') {
            let mut full = base.to_vec();
            full.extend_from_slice(name);
            out.push(full);
        }
    }
    out
}
