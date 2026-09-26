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


//! A guest's console output, carried to the host's log.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// Cap on one transfer, matching the kernel's own peer-copy ceiling.
const MAX_IO: u64 = 1 << 20;

/// A guest's console output, carried to the host's log. The bytes are the
/// guest's and are never interpreted, only forwarded.
pub(super) fn console(guest: &Guest, buf: u64, len: u64) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    let take = len.min(MAX_IO);
    let Some(bytes) = guest.read(buf, take as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let _ = nonos_libc::mk_debug(bytes.as_ptr(), bytes.len());
    errno::ok(take)
}

