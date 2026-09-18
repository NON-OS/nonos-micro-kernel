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


//! The open flags and the special directory descriptor, as Linux defines
//! them on x86_64. Transcribed, never chosen.

pub const O_WRONLY: u64 = 0o1;
pub const O_RDWR: u64 = 0o2;
pub const O_CREAT: u64 = 0o100;
pub const O_TRUNC: u64 = 0o1000;
pub const O_APPEND: u64 = 0o2000;
pub const O_DIRECTORY: u64 = 0o200000;

/// `openat` with this as the directory means "relative to the working
/// directory", which is the only relative form a static binary uses.
pub const AT_FDCWD: u64 = (-100i64) as u64;

/// Set by `newfstatat` when the caller means the link and not its target.
pub const AT_EMPTY_PATH: u64 = 0x1000;

/// A guest asked to write if it asked for anything but read.
pub fn wants_write(flags: u64) -> bool {
    flags & (O_WRONLY | O_RDWR | O_CREAT | O_TRUNC | O_APPEND) != 0
}

pub fn wants_read(flags: u64) -> bool {
    flags & O_WRONLY == 0
}
