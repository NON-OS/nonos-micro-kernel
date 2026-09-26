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

//! Waiting.

use nonos_libc::{mk_uptime_ms, mk_yield};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// `timespec` is two 64-bit words: seconds then nanoseconds.
const PAIR: usize = 16;

/// `nanosleep`: yield until the deadline passes.
pub fn nanosleep(guest: &Guest, req: u64) -> u64 {
    let Some(spec) = guest.read(req, PAIR) else {
        return errno::fail(errno::EFAULT);
    };
    let secs = u64::from_le_bytes(spec[..8].try_into().unwrap_or([0; 8]));
    let nanos = u64::from_le_bytes(spec[8..16].try_into().unwrap_or([0; 8]));
    let until = uptime().saturating_add(secs * 1000 + nanos / 1_000_000);
    while uptime() < until {
        mk_yield();
    }
    errno::ok(0)
}

fn uptime() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}
