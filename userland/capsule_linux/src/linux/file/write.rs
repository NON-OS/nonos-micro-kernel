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


//! Writing to a file a guest has open.
//!
//! Bytes are held here until the descriptor is closed, then written as one
//! file. The store takes whole values rather than a stream of positioned
//! writes, and a program that writes a file expects it to appear whole or
//! not at all, which is the same thing.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

/// One transfer, matching the kernel's own peer-copy ceiling.
const MAX_IO: u64 = 1 << 20;

/// What a single guest may hold unwritten. A program that produces more
/// than this without closing is refused rather than allowed to grow this
/// capsule's heap without bound.
const MAX_PENDING: usize = 8 << 20;

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let take = len.min(MAX_IO);
    let Some(bytes) = guest.read(buf, take as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let Some(entry) = guest.fds.get_mut(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    if entry.kind != Kind::File || !entry.writable {
        return errno::fail(errno::EBADF);
    }
    let at = entry.offset as usize;
    if at + bytes.len() > MAX_PENDING {
        return errno::fail(errno::ENOSPC);
    }
    if entry.pending.len() < at + bytes.len() {
        entry.pending.resize(at + bytes.len(), 0);
    }
    entry.pending[at..at + bytes.len()].copy_from_slice(&bytes);
    entry.offset += bytes.len() as u64;
    entry.size = entry.size.max(entry.pending.len() as u64);
    errno::ok(bytes.len() as u64)
}
