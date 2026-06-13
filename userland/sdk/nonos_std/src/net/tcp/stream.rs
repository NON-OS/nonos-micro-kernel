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

use crate::io::{Read, Result, Write};
use crate::net::addr::{resolve, ToSocketAddrs};
use crate::net::socket::{Socket, KIND_STREAM};

pub struct TcpStream {
    inner: Socket,
}

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let (ip, port) = resolve(addr)?;
        let inner = Socket::open(KIND_STREAM)?;
        inner.connect(ip, port)?;
        Ok(Self { inner })
    }

    pub(crate) fn from_socket(inner: Socket) -> Self {
        Self { inner }
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.recv(buf)
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.send(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
