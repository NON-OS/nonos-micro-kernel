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

//! What the store knows about a path, and the two calls that ask.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::flags::AT_FDCWD;
use super::super::{path, resolve, store};
use super::statbuf::{build, STAT_LEN};

/// Size and whether it is a directory, or nothing when the path is
/// absent. `full` is guest-visible and is confined here.
pub fn look(full: &[u8]) -> Option<(u64, bool)> {
    match store::stat_full(&resolve::key(full)) {
        Ok((size, is_dir, _, _)) => Some((size, is_dir)),
        Err(_) => None,
    }
}

pub fn fstat(guest: &mut Guest, fd: u64, out: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    let (size, is_dir) = match entry.kind {
        Kind::Free => return errno::fail(errno::EBADF),
        Kind::Dir => (0, true),
        Kind::File => (entry.size.max(entry.pending.len() as u64), false),
        _ => (0, false),
    };
    write_out(guest, out, size, is_dir)
}

pub fn newfstatat(guest: &mut Guest, dirfd: u64, path_ptr: u64, out: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    if dirfd != AT_FDCWD {
        return errno::fail(errno::ENOSYS);
    }
    let full = resolve::visible(&guest.cwd, &name);
    match look(&full) {
        Some((size, is_dir)) => write_out(guest, out, size, is_dir),
        None => errno::fail(errno::ENOENT),
    }
}

fn write_out(guest: &Guest, out: u64, size: u64, is_dir: bool) -> u64 {
    if guest.write(out, &build(size, is_dir)) < STAT_LEN as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
