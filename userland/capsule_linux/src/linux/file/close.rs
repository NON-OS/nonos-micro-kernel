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

//! Closing a descriptor, and writing out anything it was holding.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

pub fn close(guest: &mut Guest, fd: u64) -> u64 {
    let Some(entry) = guest.fds.get_mut(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    if !entry.is_open() {
        return errno::fail(errno::EBADF);
    }
    /*
     * The store handle is dropped with the descriptor, which closes it on the
     * server.
     */
    let flushed = flush(entry);
    *entry = Fd::empty(Kind::Free);
    match flushed {
        true => errno::ok(0),
        false => errno::fail(errno::EIO),
    }
}

/// Write a descriptor's buffered bytes out.
pub(super) fn flush(entry: &Fd) -> bool {
    if entry.kind != Kind::File || !entry.writable {
        return true;
    }
    super::store::write(&super::resolve::key(&entry.path), &entry.pending).is_ok()
}
