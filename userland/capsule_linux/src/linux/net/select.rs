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
//! `select`, answered from the same readiness the poll path reports.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::ready;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;

/// Linux caps a descriptor set at 1024 bits and so does every libc that
/// builds one, so a larger nfds is a caller error rather than a bigger set.
const FD_SETSIZE: u64 = 1024;
const SET_BYTES: usize = (FD_SETSIZE / 8) as usize;

pub fn select(guest: &mut Guest, nfds: u64, readfds: u64, writefds: u64) -> u64 {
    if nfds > FD_SETSIZE {
        return errno::fail(errno::EINVAL);
    }
    let mut hits = 0u64;
    for (at, want) in [(readfds, POLLIN), (writefds, POLLOUT)] {
        if at == 0 {
            continue;
        }
        let Some(mut set) = guest.read(at, SET_BYTES) else {
            return errno::fail(errno::EFAULT);
        };
        hits += narrow(guest, &mut set, nfds, want);
        if guest.write(at, &set) < 0 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(hits)
}

/// Clear every bit whose descriptor is not ready for `want`, and report
/// how many were left set.
fn narrow(guest: &Guest, set: &mut [u8], nfds: u64, want: u16) -> u64 {
    let mut kept = 0;
    for fd in 0..nfds {
        let (byte, bit) = ((fd / 8) as usize, (fd % 8) as u32);
        if set[byte] & (1 << bit) == 0 {
            continue;
        }
        if ready(guest, fd) & want != 0 {
            kept += 1;
        } else {
            set[byte] &= !(1 << bit);
        }
    }
    kept
}
