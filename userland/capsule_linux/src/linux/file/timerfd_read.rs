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

//! Reading a timer, and whether it has fired.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

/// A read reports how many times it has fired, which is one or none.
pub fn read(guest: &mut Guest, fd: u64, buf: u64) -> u64 {
    let now = nonos_libc::mk_uptime_ms().max(0) as u64;
    let fired = match guest.fds.get(fd as usize) {
        Some(e) if e.kind == Kind::Timer => e.expiry != 0 && now >= e.expiry,
        _ => return errno::fail(errno::EBADF),
    };
    if !fired {
        return errno::fail(errno::EAGAIN);
    }
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        entry.expiry = 0;
    }
    if guest.write(buf, &1u64.to_le_bytes()) < 8 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(8)
}
