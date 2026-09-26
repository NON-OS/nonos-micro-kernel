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

//! Moving the working directory.

use crate::linux::abi::errno;
use crate::linux::file::{look, read_path, visible};
use crate::linux::guest::{Guest, Kind};

pub fn chdir(guest: &mut Guest, path: u64) -> u64 {
    let Some(name) = read_path(guest, path) else {
        return errno::fail(errno::EFAULT);
    };
    let at = visible(&guest.cwd, &name);
    // Checked before it is taken.
    match look(&at) {
        Some(_) => {
            guest.cwd = at;
            errno::ok(0)
        }
        None => errno::fail(errno::ENOENT),
    }
}

/// `fchdir`: the same, named by a directory the guest already opened.
pub fn fchdir(guest: &mut Guest, fd: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Dir) else {
        return errno::fail(errno::EBADF);
    };
    guest.cwd = entry.path.clone();
    errno::ok(0)
}

/// `getcwd` writes the path and returns its length including the terminator,
/// which is what a libc uses to tell success from a buffer that was too small.
pub fn getcwd(guest: &Guest, buf: u64, len: u64) -> u64 {
    let mut out = guest.cwd.clone();
    out.push(0);
    if (len as usize) < out.len() {
        return errno::fail(errno::ERANGE);
    }
    match guest.write(buf, &out) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(out.len() as u64),
    }
}
