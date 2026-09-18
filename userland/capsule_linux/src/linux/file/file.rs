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


//! Opening a regular file, and creating one that is not there yet.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::flags::{wants_read, wants_write, O_TRUNC};
use super::slot;

pub fn open(guest: &mut Guest, owner: u32, path: Vec<u8>, size: u64, flags: u64) -> u64 {
    // No server handle for write-only or O_TRUNC: nothing will read it.
    let truncating = flags & O_TRUNC != 0;
    let stream = if wants_read(flags) && !truncating {
        match VfsStream::open(owner, &path) {
            Ok(s) => Some(s),
            Err(_) => return errno::fail(errno::EACCES),
        }
    } else {
        None
    };
    let size = if truncating { 0 } else { size };
    let fd = Fd::file(path, size, stream, wants_write(flags));
    match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/// Nothing hits the store until close, so a create-then-die leaves no file.
pub fn create(guest: &mut Guest, path: Vec<u8>) -> u64 {
    let fd = Fd::file(path, 0, None, true);
    match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
