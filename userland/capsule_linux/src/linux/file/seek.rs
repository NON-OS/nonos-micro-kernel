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


//! `lseek`. The position is this capsule's, not the server's: a read takes
//! a window at an offset, so the descriptor's offset is the whole of it.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

const SEEK_SET: u64 = 0;
const SEEK_CUR: u64 = 1;
const SEEK_END: u64 = 2;

pub fn lseek(guest: &mut Guest, fd: u64, offset: u64, whence: u64) -> u64 {
    let Some(entry) = guest.fds.get_mut(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    if entry.kind != Kind::File {
        // A pipe or a console has no position, which Linux calls ESPIPE.
        return errno::fail(errno::ESPIPE);
    }
    let delta = offset as i64;
    let base = match whence {
        SEEK_SET => 0,
        SEEK_CUR => entry.offset as i64,
        SEEK_END => entry.size as i64,
        _ => return errno::fail(errno::EINVAL),
    };
    let Some(at) = base.checked_add(delta).filter(|v| *v >= 0) else {
        return errno::fail(errno::EINVAL);
    };
    entry.offset = at as u64;
    errno::ok(entry.offset)
}
