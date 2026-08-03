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

//! Negative-errno values used by microkernel syscall handlers. The
//! sign convention is `-errno`; the syscall return value is `i64` so
//! a successful call returns a non-negative number and a failure
//! returns one of these constants.

pub const ERRNO_PERM: i64 = -1;
pub const ERRNO_NOENT: i64 = -2;
pub const ERRNO_CHILD: i64 = -10;
pub const ERRNO_NOMEM: i64 = -12;
pub const ERRNO_ACCES: i64 = -13;
pub const ERRNO_FAULT: i64 = -14;
pub const ERRNO_BUSY: i64 = -16;
pub const ERRNO_EXIST: i64 = -17;
pub const ERRNO_NODEV: i64 = -19;
pub const ERRNO_INVAL: i64 = -22;
pub const ERRNO_NOSYS: i64 = -38;
pub const ERRNO_NOTSUP: i64 = -95;
pub const ERRNO_TIMEDOUT: i64 = -110;
pub const ERRNO_STALE: i64 = -116;
