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


//! `openat`, and the plain `open` older binaries still use.
//!
//! Everything a guest opens is opened under this capsule's own identity,
//! never the guest's: a hosted process holds no capabilities at all, so
//! the store would refuse it outright. A guest therefore reaches exactly
//! what this capsule is allowed to reach, and narrowing that is a matter
//! of what the capsule is granted, not of anything the guest can ask for.

use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::flags::{wants_write, AT_FDCWD, O_CREAT, O_DIRECTORY};
use super::{dir, file, path, resolve, stat};

pub fn openat(guest: &mut Guest, dirfd: u64, path_ptr: u64, flags: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let base = match base_of(guest, dirfd) {
        Ok(base) => base,
        Err(e) => return e,
    };
    let full = resolve::absolute(&base, &name);
    let owner = mk_getpid() as u32;
    match stat::look(owner, &full) {
        Some((_, true)) => dir::open(guest, owner, full),
        Some((_, false)) if flags & O_DIRECTORY != 0 => errno::fail(errno::ENOTDIR),
        Some((size, false)) => file::open(guest, owner, full, size, flags),
        None if flags & O_CREAT != 0 && wants_write(flags) => file::create(guest, full),
        None => errno::fail(errno::ENOENT),
    }
}

/// The directory a relative path is relative to. Only the working
/// directory and a directory the guest opened itself can be named.
fn base_of(guest: &Guest, dirfd: u64) -> Result<Vec<u8>, u64> {
    if dirfd == AT_FDCWD {
        return Ok(guest.cwd.clone());
    }
    match guest.fds.get(dirfd as usize) {
        Some(fd) if fd.kind == Kind::Dir => Ok(fd.path.clone()),
        Some(_) => Err(errno::fail(errno::ENOTDIR)),
        None => Err(errno::fail(errno::EBADF)),
    }
}
