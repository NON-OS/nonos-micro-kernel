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


//! `sendmsg` and `recvmsg`, which is how libwayland actually talks.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::wayland;

use super::msg::read_msghdr;
use super::sock::connected;

pub fn sendmsg(guest: &mut Guest, fd: u64, at: u64) -> u64 {
    if !connected(guest, fd) {
        return errno::fail(errno::ENOTCONN);
    }
    let Some(msg) = read_msghdr(guest, at) else {
        return errno::fail(errno::EFAULT);
    };
    let sent = msg.bytes.len() as u64;
    guest.display.fds.extend_from_slice(&msg.fds);
    guest.display.to_server.extend_from_slice(&msg.bytes);
    wayland::serve(guest);
    errno::ok(sent)
}
