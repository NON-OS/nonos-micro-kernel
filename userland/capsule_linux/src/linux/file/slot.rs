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


//! Choosing the number a new descriptor gets.
//!
//! Linux promises the lowest free number, and programs depend on it: the
//! shell idiom of closing a descriptor and immediately opening a file to
//! take its place is exactly that promise.

use crate::linux::guest::{Fd, Guest};

/// More than any single hosted program needs, and low enough that a
/// descriptor leak shows up as a refusal rather than as memory growth.
pub const MAX_FDS: usize = 256;

pub fn install(guest: &mut Guest, fd: Fd) -> Option<u64> {
    for (i, slot) in guest.fds.iter_mut().enumerate() {
        if !slot.is_open() {
            *slot = fd;
            return Some(i as u64);
        }
    }
    if guest.fds.len() >= MAX_FDS {
        return None;
    }
    guest.fds.push(fd);
    Some((guest.fds.len() - 1) as u64)
}
