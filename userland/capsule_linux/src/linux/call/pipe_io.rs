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

//! Bytes through a pipe.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::pipe_end::end_of;

/// What one pipe will hold before a writer is told to wait. Linux uses
/// sixty-four kilobytes and programs are written around that number.
const CAPACITY: usize = 64 << 10;

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some((slot, writable)) = end_of(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    if !writable {
        return errno::fail(errno::EBADF);
    }
    let room = CAPACITY.saturating_sub(guest.pipes[slot].len());
    if room == 0 {
        // A full pipe blocks on Linux until a reader drains it.
        return errno::fail(errno::EAGAIN);
    }
    let take = (len as usize).min(room);
    let Some(bytes) = guest.read(buf, take) else {
        return errno::fail(errno::EFAULT);
    };
    guest.pipes[slot].extend_from_slice(&bytes);
    errno::ok(take as u64)
}
