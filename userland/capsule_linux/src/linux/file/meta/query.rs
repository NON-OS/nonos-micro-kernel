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

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::{path, resolve};
use super::stat;

pub fn access(guest: &Guest, path_ptr: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let full = resolve::visible(&guest.cwd, &name);
    match stat::look(&full) {
        Some(_) => errno::ok(0),
        None => errno::fail(errno::ENOENT),
    }
}

/// The store holds no symbolic links, so a path that exists is not one and a
/// path that does not exist is absent.
pub fn readlink(guest: &Guest, path_ptr: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let full = resolve::visible(&guest.cwd, &name);
    match stat::look(&full) {
        Some(_) => errno::fail(errno::EINVAL),
        None => errno::fail(errno::ENOENT),
    }
}
