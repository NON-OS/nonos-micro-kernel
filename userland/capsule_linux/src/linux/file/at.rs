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

//! A `dirfd` and a path, resolved to one absolute name.

use alloc::vec::Vec;

use crate::linux::guest::{Guest, Kind};

use super::flags::AT_FDCWD;
use super::path::read_path;
use super::resolve::visible;

/// The guest-visible absolute path `dirfd` and `path` name together, or `None`
/// when the path cannot be read or the descriptor is not a directory this
/// guest opened.
pub fn resolve_at(guest: &Guest, dirfd: u64, path: u64) -> Option<Vec<u8>> {
    let name = read_path(guest, path)?;
    if name.first() == Some(&b'/') {
        return Some(visible(b"/", &name));
    }
    let base = base_of(guest, dirfd)?;
    Some(visible(&base, &name))
}

fn base_of(guest: &Guest, dirfd: u64) -> Option<Vec<u8>> {
    if dirfd == AT_FDCWD {
        return Some(guest.cwd.clone());
    }
    match guest.fds.get(dirfd as usize) {
        Some(fd) if fd.kind == Kind::Dir => Some(fd.path.clone()),
        _ => None,
    }
}
