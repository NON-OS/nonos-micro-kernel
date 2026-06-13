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

use super::stream::TcpStream;
use crate::io::Result;
use crate::net::addr::{resolve, ToSocketAddrs};
use crate::net::socket::{Socket, KIND_STREAM};

pub struct TcpListener {
    inner: Socket,
}

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let (ip, port) = resolve(addr)?;
        let inner = Socket::open(KIND_STREAM)?;
        inner.bind(ip, port)?;
        inner.listen()?;
        Ok(Self { inner })
    }

    pub fn accept(&self) -> Result<TcpStream> {
        let child = self.inner.accept()?;
        Ok(TcpStream::from_socket(child))
    }
}
