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

// Establishing a TcpStream: walk the resolved addresses, open a socket and
// issue OP_CONNECT for each until one succeeds. connect_timeout ignores its
// deadline (the connect op is synchronous in the userland stack).

use super::TcpStream;
use crate::io;
use crate::net::{SocketAddr, ToSocketAddrs};
use crate::sys::net::connection::nonos::transport::{
    OP_CONNECT, close, endpoint, err, open_socket, sk, v4_parts,
};
use crate::time::Duration;

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            match Self::connect_addr(&a) {
                Ok(s) => return Ok(s),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    fn connect_addr(addr: &SocketAddr) -> io::Result<TcpStream> {
        let (ip, port) = v4_parts(addr)?;
        let handle = open_socket(1)?;
        if let Err(e) = sk(OP_CONNECT, &endpoint(handle, ip, port), 0) {
            close(handle);
            return Err(e);
        }
        Ok(Self::wrap(handle, *addr))
    }

    pub fn connect_timeout(addr: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        Self::connect_addr(addr)
    }
}
