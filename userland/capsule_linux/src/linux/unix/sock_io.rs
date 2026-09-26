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


//! Bytes to and from the display socket.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::wayland;

use super::sock::connected;

/// A write is a batch of requests.
pub fn send(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let take = len.min(1 << 20);
    let Some(bytes) = guest.read(buf, take as usize) else {
        return errno::fail(errno::EFAULT);
    };
    if !connected(guest, fd) {
        return errno::fail(errno::ENOTCONN);
    }
    guest.display.to_server.extend_from_slice(&bytes);
    wayland::serve(guest);
    errno::ok(take)
}

pub fn recv(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    if !connected(guest, fd) {
        return errno::fail(errno::ENOTCONN);
    }
    wayland::pump(guest);
    let bytes = guest.display.drain(len as usize);
    if bytes.is_empty() {
        // Non-blocking is what a toolkit asks for; it polls.
        return errno::fail(errno::EAGAIN);
    }
    if guest.write(buf, &bytes) < bytes.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(bytes.len() as u64)
}
