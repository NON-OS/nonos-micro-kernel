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

//! Process groups and sessions.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub fn setpgid(guest: &mut Guest, pid: u64, pgid: u64) -> u64 {
    let target = match pid {
        0 => guest.pid,
        n => n as u32,
    };
    if !guest.owns(target) {
        return errno::fail(errno::ESRCH);
    }
    guest.pgid = match pgid {
        0 => target,
        n => n as u32,
    };
    errno::ok(0)
}

pub fn getpgid(guest: &Guest) -> u64 {
    errno::ok(u64::from(guest.pgid))
}

/// `setsid` makes the caller a session leader, which means a new group with
/// its own id.
pub fn setsid(guest: &mut Guest) -> u64 {
    if guest.sid == guest.pid {
        return errno::fail(errno::EPERM);
    }
    guest.sid = guest.pid;
    guest.pgid = guest.pid;
    errno::ok(u64::from(guest.sid))
}

pub fn getsid(guest: &Guest) -> u64 {
    errno::ok(u64::from(guest.sid))
}
