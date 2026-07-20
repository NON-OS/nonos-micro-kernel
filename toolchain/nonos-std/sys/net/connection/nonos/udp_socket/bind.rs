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

// Binding a UDP socket: walk the resolved addresses, open a datagram socket
// and issue OP_BIND for each until one takes.

use super::UdpSocket;
use crate::io;
use crate::net::ToSocketAddrs;
use crate::sys::net::connection::nonos::transport::{
    OP_BIND, close, endpoint, err, open_socket, sk, v4_parts,
};

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            let (ip, port) = match v4_parts(&a) {
                Ok(v) => v,
                Err(e) => {
                    last = e;
                    continue;
                }
            };
            let handle = open_socket(2)?;
            match sk(OP_BIND, &endpoint(handle, ip, port), 0) {
                Ok(_) => return Ok(UdpSocket { handle, local: a }),
                Err(e) => {
                    close(handle);
                    last = e;
                }
            }
        }
        Err(last)
    }
}
