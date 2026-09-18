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


//! `getcwd`, `access` and `readlink`: the three questions a program asks
//! about a path without opening it.

use nonos_libc::mk_getpid;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::{path, resolve, stat};

pub fn getcwd(guest: &Guest, out: u64, size: u64) -> u64 {
    let need = guest.cwd.len() + 1;
    if size < need as u64 {
        return errno::fail(errno::ERANGE);
    }
    let mut buf = guest.cwd.clone();
    buf.push(0);
    if guest.write(out, &buf) < need as i64 {
        return errno::fail(errno::EFAULT);
    }
    /* Linux returns the length including the terminator, not a pointer. */
    errno::ok(need as u64)
}

pub fn access(guest: &Guest, path_ptr: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let full = resolve::absolute(&guest.cwd, &name);
    match stat::look(mk_getpid() as u32, &full) {
        Some(_) => errno::ok(0),
        None => errno::fail(errno::ENOENT),
    }
}

/// The store holds no symbolic links, so a path that exists is not one
/// and a path that does not exist is absent. Both are real answers, and
/// neither is the invented target a caller would act on.
pub fn readlink(guest: &Guest, path_ptr: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let full = resolve::absolute(&guest.cwd, &name);
    match stat::look(mk_getpid() as u32, &full) {
        Some(_) => errno::fail(errno::EINVAL),
        None => errno::fail(errno::ENOENT),
    }
}
