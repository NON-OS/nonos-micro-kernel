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

//! `ioctl` and `fcntl`.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const F_DUPFD: u64 = 0;
const F_GETFD: u64 = 1;
const F_SETFD: u64 = 2;
const F_GETFL: u64 = 3;
const F_SETFL: u64 = 4;

pub fn ioctl(guest: &Guest, fd: u64, _request: u64) -> u64 {
    match guest.fds.get(fd as usize) {
        Some(entry) if entry.is_open() => errno::fail(errno::ENOTTY),
        _ => errno::fail(errno::EBADF),
    }
}

/// The only descriptor flag there is.
const FD_CLOEXEC: u64 = 1;

pub fn fcntl(guest: &mut Guest, fd: u64, cmd: u64, arg: u64) -> u64 {
    let Some(entry) = guest.fds.get_mut(fd as usize).filter(|e| e.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    match cmd {
        /*
         * A shell sets close-on-exec on the descriptors it keeps for itself,
         * then execs, and expects the command not to see them.
         */
        F_GETFD => errno::ok(u64::from(entry.cloexec)),
        F_SETFD => {
            entry.cloexec = arg & FD_CLOEXEC != 0;
            errno::ok(0)
        }
        /*
         * Reported as the read-write the descriptor already has; a request to
         * change them is accepted because none of the flags a program sets
         * here has an effect.
         */
        F_SETFL => errno::ok(0),
        F_GETFL => errno::ok(2),
        /*
         * Duplication needs a second handle on the server, which the store
         * does not offer yet.
         */
        F_DUPFD => errno::fail(errno::ENOSYS),
        _ => errno::fail(errno::EINVAL),
    }
}
