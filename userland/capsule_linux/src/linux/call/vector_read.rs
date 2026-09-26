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
//! `readv`, the reading half of the scatter-gather pair.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// One `struct iovec`: a pointer and a length, both eight bytes.
const IOVEC: usize = 16;
/// Linux refuses a longer vector, so a guest cannot ask this capsule to
/// walk an unbounded list.
const IOV_MAX: u64 = 1024;

pub fn readv(guest: &mut Guest, fd: u64, iov: u64, count: u64) -> u64 {
    if count > IOV_MAX {
        return errno::fail(errno::EINVAL);
    }
    let Some(table) = guest.read(iov, count as usize * IOVEC) else {
        return errno::fail(errno::EFAULT);
    };
    let mut got = 0u64;
    for i in 0..count as usize {
        let at = i * IOVEC;
        let (base, len) = (word(&table, at), word(&table, at + 8));
        if len == 0 {
            continue;
        }
        let result = super::io::read(guest, fd, base, len);
        if (result as i64) < 0 {
            /*
             * A failure after a partial read is that partial count: the bytes
             * already in the guest's buffers are real and a caller told
             * otherwise would read them twice.
             */
            return if got == 0 { result } else { errno::ok(got) };
        }
        got += result;
        // A short read ends the vector.
        if result < len {
            break;
        }
    }
    errno::ok(got)
}

fn word(bytes: &[u8], at: usize) -> u64 {
    match bytes.get(at..at + 8).and_then(|s| s.try_into().ok()) {
        Some(eight) => u64::from_le_bytes(eight),
        None => 0,
    }
}
