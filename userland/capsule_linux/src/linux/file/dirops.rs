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

//! Making, removing and moving names in the store.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::resolve::key;
use super::store_name;

pub fn mkdirat(guest: &Guest, dirfd: u64, path: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    match store_name::mkdir(&key(&at)) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::EEXIST),
    }
}

pub fn rmdir(guest: &Guest, path: u64) -> u64 {
    let Some(at) = resolve_at(guest, super::flags::AT_FDCWD, path) else {
        return errno::fail(errno::EFAULT);
    };
    /*
     * Not recursive: POSIX rmdir refuses a populated directory, and a
     * recursive delete behind that name is data loss.
     */
    match store_name::rmdir(&key(&at)) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOTEMPTY),
    }
}

pub fn unlinkat(guest: &Guest, dirfd: u64, path: u64, flags: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    /*
     * AT_REMOVEDIR turns unlinkat into rmdir, which is how a libc implements
     * rmdir on top of one syscall.
     */
    const AT_REMOVEDIR: u64 = 0x200;
    let at = key(&at);
    let done = match flags & AT_REMOVEDIR {
        0 => store_name::unlink(&at),
        _ => store_name::rmdir(&at),
    };
    match done {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}
