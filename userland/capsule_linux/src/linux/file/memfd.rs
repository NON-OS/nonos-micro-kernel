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

//! `memfd_create` and `ftruncate`.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::slot::install;

pub fn is_memfd(guest: &Guest, fd: u64) -> bool {
    matches!(guest.fds.get(fd as usize), Some(e) if e.kind == Kind::Memfd)
}

pub fn memfd_create(guest: &mut Guest) -> u64 {
    match install(guest, Fd::memfd()) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/// Sizing one records the size and nothing else. The pages appear when
/// the client maps it, because that is when their address is decided.
pub fn ftruncate(guest: &mut Guest, fd: u64, len: u64) -> u64 {
    match guest.fds.get_mut(fd as usize) {
        Some(entry) if entry.kind == Kind::Memfd => {
            entry.size = len;
            errno::ok(0)
        }
        Some(_) => errno::fail(errno::EINVAL),
        None => errno::fail(errno::EBADF),
    }
}
