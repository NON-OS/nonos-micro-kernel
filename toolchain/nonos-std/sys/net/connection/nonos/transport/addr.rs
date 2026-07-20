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

// Address helpers: split a SocketAddr into IPv4 octets and a port (IPv6 is
// unsupported by the userland stack), pack a handle+ip+port endpoint the
// bind/connect ops carry, and the unspecified 0.0.0.0:0 address returned where
// the protocol reports no peer.

use super::err::err;
use crate::io;
use crate::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub(crate) fn v4_parts(addr: &SocketAddr) -> io::Result<([u8; 4], u16)> {
    match addr {
        SocketAddr::V4(a) => Ok((a.ip().octets(), a.port())),
        SocketAddr::V6(_) => Err(err("ipv6 unsupported")),
    }
}

pub(crate) fn endpoint(handle: u32, ip: [u8; 4], port: u16) -> [u8; 10] {
    let mut b = [0u8; 10];
    b[0..4].copy_from_slice(&handle.to_le_bytes());
    b[4..8].copy_from_slice(&ip);
    b[8..10].copy_from_slice(&port.to_le_bytes());
    b
}

pub(crate) fn unspecified() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0))
}
