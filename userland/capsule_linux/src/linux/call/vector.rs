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

//! `writev`, which a C runtime reaches for as often as `write`. The vector
//! is an array of pointer and length pairs in the guest's memory, so it is
//! read out of the guest before any of it is followed.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// One `struct iovec`: a pointer and a length, both eight bytes.
const IOVEC: usize = 16;
/// Linux refuses a vector longer than this, so a guest cannot ask this
/// capsule to walk an unbounded list.
const IOV_MAX: u64 = 1024;

pub fn writev(guest: &mut Guest, fd: u64, iov: u64, count: u64) -> u64 {
    if count > IOV_MAX {
        return errno::fail(errno::EINVAL);
    }
    let Some(table) = guest.read(iov, count as usize * IOVEC) else {
        return errno::fail(errno::EFAULT);
    };
    let mut written = 0u64;
    for i in 0..count as usize {
        let at = i * IOVEC;
        let base = word(&table, at);
        let len = word(&table, at + 8);
        if len == 0 {
            continue;
        }
        let result = super::io::write(guest, fd, base, len);
        if (result as i64) < 0 {
            // A failure after a partial write is that partial count.
            return if written == 0 { result } else { errno::ok(written) };
        }
        written += result;
        if result < len {
            break;
        }
    }
    errno::ok(written)
}

fn word(bytes: &[u8], at: usize) -> u64 {
    match bytes.get(at..at + 8).and_then(|s| s.try_into().ok()) {
        Some(eight) => u64::from_le_bytes(eight),
        None => 0,
    }
}
