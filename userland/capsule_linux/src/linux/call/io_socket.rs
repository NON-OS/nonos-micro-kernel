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


//! Reads and writes that land on a socket rather than a file.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::net;

pub(super) fn socket_write(guest: &Guest, fd: u64, buf: u64, len: u64) -> u64 {
    match guest.socket_handle(fd) {
        Some(h) => net::send(guest, h, buf, len),
        None => errno::fail(errno::EBADF),
    }
}

pub(super) fn socket_read(guest: &Guest, fd: u64, buf: u64, len: u64) -> u64 {
    match guest.socket_handle(fd) {
        Some(h) => net::recv(guest, h, buf, len),
        None => errno::fail(errno::EBADF),
    }
}

