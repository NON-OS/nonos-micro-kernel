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

//! `timerfd_create`, `timerfd_settime`, and reading one.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::slot::install;

/// `struct itimerspec`: interval seconds and nanoseconds, then the
/// value's seconds and nanoseconds. Four eight byte fields.
const ITIMERSPEC_LEN: usize = 32;

pub fn timerfd_create(guest: &mut Guest) -> u64 {
    match install(guest, Fd::empty(Kind::Timer)) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

pub fn timerfd_settime(guest: &mut Guest, fd: u64, spec: u64) -> u64 {
    let Some(raw) = guest.read(spec, ITIMERSPEC_LEN) else {
        return errno::fail(errno::EFAULT);
    };
    let secs = u64::from_le_bytes(raw[16..24].try_into().unwrap_or([0; 8]));
    let nanos = u64::from_le_bytes(raw[24..32].try_into().unwrap_or([0; 8]));
    let delay = secs * 1000 + nanos / 1_000_000;
    let now = nonos_libc::mk_uptime_ms().max(0) as u64;
    match guest.fds.get_mut(fd as usize).filter(|f| f.kind == Kind::Timer) {
        Some(entry) => {
            entry.expiry = if delay == 0 { 0 } else { now + delay };
            errno::ok(0)
        }
        None => errno::fail(errno::EBADF),
    }
}
