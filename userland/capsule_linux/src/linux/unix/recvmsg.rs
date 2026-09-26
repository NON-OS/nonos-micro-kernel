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


//! `recvmsg` on the display socket.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::msg::{u64le, MSGHDR_LEN};
use super::sock::connected;

/// The reply goes into the first iovec only.
pub fn recvmsg(guest: &mut Guest, fd: u64, at: u64) -> u64 {
    if !connected(guest, fd) {
        return errno::fail(errno::ENOTCONN);
    }
    let Some(raw) = guest.read(at, MSGHDR_LEN) else {
        return errno::fail(errno::EFAULT);
    };
    let Some((base, len)) = first_iov(guest, &raw) else {
        return errno::fail(errno::EFAULT);
    };
    crate::linux::wayland::pump(guest);
    if let Some(block) = super::give::control(guest) {
        super::give::place(guest, at, &block);
    }
    let bytes = guest.display.drain(len as usize);
    if bytes.is_empty() {
        return errno::fail(errno::EAGAIN);
    }
    if guest.write(base, &bytes) < bytes.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(bytes.len() as u64)
}

fn first_iov(guest: &Guest, hdr: &[u8]) -> Option<(u64, u64)> {
    let iov = u64le(hdr, 16);
    let count = u64le(hdr, 24);
    if count == 0 {
        return None;
    }
    let entry = guest.read(iov, 16)?;
    Some((u64le(&entry, 0), u64le(&entry, 8)))
}
