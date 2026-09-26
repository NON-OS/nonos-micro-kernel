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

//! `connect`, for a socket the guest opened here.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::addr::inet;
use super::call::call;
use super::dns::host_for;
use super::ops::{OP_CONNECT, OP_CONNECT_HOST};

pub fn connect(guest: &mut Guest, fd: u64, at: u64, len: u64) -> u64 {
    /*
     * A program may connect its nameserver socket before writing to
     * it. There is nothing to reach: this capsule is the nameserver.
     */
    if guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Resolver) {
        return errno::ok(0);
    }
    let Some(handle) = guest.socket_handle(fd) else {
        return errno::fail(errno::ENOTSOCK);
    };
    let Some((port, ip)) = inet(guest, at, len) else {
        return errno::fail(errno::EAFNOSUPPORT);
    };
    // An address this capsule invented for a name goes back to being the name.
    if let Some(host) = host_for(guest, ip) {
        return by_host(handle, &host, port);
    }
    let mut body = Vec::with_capacity(10);
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&ip);
    body.extend_from_slice(&port.to_le_bytes());
    match call(OP_CONNECT, &body, 0) {
        Some((0, _)) => errno::ok(0),
        Some(_) => errno::fail(errno::ECONNREFUSED),
        None => errno::fail(errno::EIO),
    }
}

fn by_host(handle: u32, host: &[u8], port: u16) -> u64 {
    if host.len() > u8::MAX as usize {
        return errno::fail(errno::EINVAL);
    }
    let mut body = Vec::with_capacity(7 + host.len());
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&port.to_le_bytes());
    body.push(host.len() as u8);
    body.extend_from_slice(host);
    match call(OP_CONNECT_HOST, &body, 0) {
        Some((0, _)) => errno::ok(0),
        Some(_) => errno::fail(errno::ECONNREFUSED),
        None => errno::fail(errno::EIO),
    }
}
