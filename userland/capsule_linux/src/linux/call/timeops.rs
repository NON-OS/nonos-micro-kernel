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

//! The clock, as a Linux program asks for it.

use nonos_libc::mk_time_millis;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// `timespec` and `timeval` are both two 64-bit words; they differ only
/// in whether the second is nanoseconds or microseconds.
const PAIR: usize = 16;

fn now_ms() -> u64 {
    u64::try_from(mk_time_millis()).unwrap_or(0)
}

/// `time`: whole seconds since the epoch, returned and optionally stored.
pub fn time(guest: &Guest, out: u64) -> u64 {
    let secs = now_ms() / 1000;
    if out != 0 && guest.write(out, &secs.to_le_bytes()) < 0 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(secs)
}

pub fn gettimeofday(guest: &Guest, tv: u64) -> u64 {
    if tv == 0 {
        return errno::ok(0);
    }
    let ms = now_ms();
    let mut buf = [0u8; PAIR];
    buf[..8].copy_from_slice(&(ms / 1000).to_le_bytes());
    buf[8..].copy_from_slice(&((ms % 1000) * 1000).to_le_bytes());
    match guest.write(tv, &buf) {
        n if n < 0 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}

