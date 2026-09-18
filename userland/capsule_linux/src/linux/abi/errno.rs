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

//! Linux errno values, and the convention for returning them.
//!
//! A Linux syscall reports failure as a small negative number in `rax`,
//! not as an errno variable. These are the values a guest expects to see,
//! which are Linux's and not this system's.

pub const EPERM: i64 = 1;
pub const ENOENT: i64 = 2;
pub const EIO: i64 = 5;
pub const EBADF: i64 = 9;
pub const ECHILD: i64 = 10;
pub const EAGAIN: i64 = 11;
pub const ENOMEM: i64 = 12;
pub const EACCES: i64 = 13;
pub const EFAULT: i64 = 14;
pub const EBUSY: i64 = 16;
pub const EEXIST: i64 = 17;
pub const ENODEV: i64 = 19;
pub const ENOTDIR: i64 = 20;
pub const ENOSPC: i64 = 28;
pub const EISDIR: i64 = 21;
pub const EINVAL: i64 = 22;
pub const ENFILE: i64 = 23;
pub const EMFILE: i64 = 24;
pub const ENOTTY: i64 = 25;
pub const ESPIPE: i64 = 29;
pub const EPIPE: i64 = 32;
pub const ERANGE: i64 = 34;
pub const ENOSYS: i64 = 38;
pub const ECONNRESET: i64 = 104;
pub const ENOTSOCK: i64 = 88;
pub const ENOTSUP: i64 = 95;
pub const EAFNOSUPPORT: i64 = 97;
pub const ECONNREFUSED: i64 = 111;
pub const EINPROGRESS: i64 = 115;

/// The value a guest's `rax` receives for a failure.
pub fn fail(errno: i64) -> u64 {
    (-errno) as u64
}

/// The value a guest's `rax` receives for a success carrying a count or a
/// descriptor.
pub fn ok(value: u64) -> u64 {
    value
}
