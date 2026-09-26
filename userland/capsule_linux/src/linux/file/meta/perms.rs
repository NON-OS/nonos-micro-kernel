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

//! Mode bits, and whether a path can be reached.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::at::resolve_at;
use super::super::flags::AT_FDCWD;
use super::super::resolve::key;
use super::super::{store, store_name};

pub fn fchmodat(guest: &Guest, dirfd: u64, path: u64, mode: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    match store_name::chmod(&key(&at), mode as u16) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}

/// `fchmod` names the file by a descriptor the guest already holds, so
/// the path comes from the descriptor rather than from the caller.
pub fn fchmod(guest: &Guest, fd: u64, mode: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    if entry.kind != Kind::File && entry.kind != Kind::Dir {
        return errno::fail(errno::EINVAL);
    }
    match store_name::chmod(&key(&entry.path), mode as u16) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}

/// `faccessat`: does the path exist and is it reachable.
pub fn faccessat(guest: &Guest, dirfd: u64, path: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    match store::stat(&key(&at)) {
        Ok(_) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}

pub fn chmod(guest: &Guest, path: u64, mode: u64) -> u64 {
    fchmodat(guest, AT_FDCWD, path, mode)
}
