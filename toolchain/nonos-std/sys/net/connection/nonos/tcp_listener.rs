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

// A listening TCP socket over net.sockets: bind then listen on a resolved
// address, and accept a connection into a fresh TcpStream. The handle is
// owned through a `Socket` descriptor so close is RAII in the fd table. The
// local address is remembered for socket_addr; ttl and the v6/nonblocking
// options the userland stack does not track report fixed values.

use crate::fmt;
use crate::io;
use crate::net::{SocketAddr, ToSocketAddrs};
use crate::sys::FromInner;
use crate::sys::net::connection::nonos::socket::Socket;
use crate::sys::net::connection::nonos::tcp_stream::TcpStream;
use crate::sys::net::connection::nonos::transport::{
    BODY, OP_ACCEPT, OP_BIND, OP_LISTEN, close, endpoint, err, open_socket, read_u32, sk,
    unspecified, v4_parts,
};

pub struct TcpListener {
    socket: Socket,
    local: SocketAddr,
}

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            match Self::bind_addr(&a) {
                Ok(l) => return Ok(l),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    fn bind_addr(addr: &SocketAddr) -> io::Result<TcpListener> {
        let (ip, port) = v4_parts(addr)?;
        let handle = open_socket(1)?;
        if let Err(e) = sk(OP_BIND, &endpoint(handle, ip, port), 0) {
            close(handle);
            return Err(e);
        }
        if let Err(e) = sk(OP_LISTEN, &handle.to_le_bytes(), 0) {
            close(handle);
            return Err(e);
        }
        Ok(TcpListener { socket: Socket::register(handle)?, local: *addr })
    }

    pub(crate) fn socket(&self) -> &Socket {
        &self.socket
    }

    pub(crate) fn into_socket(self) -> Socket {
        self.socket
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local)
    }
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let handle = self.socket.handle()?;
        let child = read_u32(&sk(OP_ACCEPT, &handle.to_le_bytes(), 8)?, BODY);
        Ok((TcpStream::wrap(child, unspecified())?, unspecified()))
    }
    pub fn duplicate(&self) -> io::Result<TcpListener> {
        crate::sys::unsupported()
    }
    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn only_v6(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
}

impl FromInner<Socket> for TcpListener {
    // Reconstruction from a bare descriptor: the bound address is not
    // recoverable from the table, so socket_addr reads as unspecified.
    fn from_inner(socket: Socket) -> TcpListener {
        TcpListener { socket, local: unspecified() }
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpListener").field("local", &self.local).finish()
    }
}
