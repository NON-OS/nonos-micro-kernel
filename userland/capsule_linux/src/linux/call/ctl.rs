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
//!
//! There is no terminal behind any descriptor here, so every ioctl is
//! refused with ENOTTY. That is not a gap: a libc asks TCGETS to find out
//! whether stdout is a terminal, and ENOTTY is the true answer. Claiming
//! otherwise would put the program into line buffering on something that
//! is not a line.

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

pub fn fcntl(guest: &Guest, fd: u64, cmd: u64) -> u64 {
    let open = matches!(guest.fds.get(fd as usize), Some(e) if e.is_open());
    if !open {
        return errno::fail(errno::EBADF);
    }
    match cmd {
        /*
         * Nothing here is ever handed to an exec, so the close-on-exec
         * flag is genuinely clear and setting it changes nothing. The
         * status flags are reported as the read-write the descriptor
         * already has, and a request to change them is accepted because
         * none of the flags a program sets here has an effect.
         */
        F_GETFD | F_SETFD | F_SETFL => errno::ok(0),
        F_GETFL => errno::ok(2),
        /* Duplication needs a second handle on the server, which the
         * store does not offer yet. Refused rather than aliased. */
        F_DUPFD => errno::fail(errno::ENOSYS),
        _ => errno::fail(errno::EINVAL),
    }
}
