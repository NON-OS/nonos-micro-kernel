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

use crate::io::Result;
use crate::net::addr::{resolve, ToSocketAddrs};
use crate::net::socket::{Socket, KIND_DGRAM};

pub struct UdpSocket {
    inner: Socket,
}

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let (ip, port) = resolve(addr)?;
        let inner = Socket::open(KIND_DGRAM)?;
        inner.bind(ip, port)?;
        Ok(Self { inner })
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> Result<()> {
        let (ip, port) = resolve(addr)?;
        self.inner.connect(ip, port)
    }

    pub fn send(&self, buf: &[u8]) -> Result<usize> {
        self.inner.send(buf)
    }

    pub fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        self.inner.recv(buf)
    }
}
