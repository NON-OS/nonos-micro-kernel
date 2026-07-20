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

// UDP datagram I/O: recv/recv_from and send/send_to over the socket handle,
// plus connect (set the default peer). send_to points the socket at the
// destination with OP_CONNECT first. peek and peek_from are unsupported (the
// net stack has no peek yet); the source address is not reported, so recv_from
// returns the unspecified address.

use super::UdpSocket;
use crate::io;
use crate::net::{SocketAddr, ToSocketAddrs};
use crate::sys::net::connection::nonos::transport::{
    OP_CONNECT, endpoint, err, recv_on, send_on, sk, unspecified, v4_parts,
};
use crate::sys::unsupported;

impl UdpSocket {
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        let n = recv_on(self.handle, buf)?;
        Ok((n, unspecified()))
    }
    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }
    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        let (ip, port) = v4_parts(addr)?;
        sk(OP_CONNECT, &endpoint(self.handle, ip, port), 0)?;
        send_on(self.handle, buf)
    }
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        recv_on(self.handle, buf)
    }
    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        send_on(self.handle, buf)
    }
    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            let (ip, port) = match v4_parts(&a) {
                Ok(v) => v,
                Err(e) => {
                    last = e;
                    continue;
                }
            };
            match sk(OP_CONNECT, &endpoint(self.handle, ip, port), 0) {
                Ok(_) => return Ok(()),
                Err(e) => last = e,
            }
        }
        Err(last)
    }
}
