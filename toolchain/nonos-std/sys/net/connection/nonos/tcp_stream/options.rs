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

// The address queries and socket options std exposes on a TcpStream. peer_addr
// returns the connected peer; the local address, linger, nodelay and ttl are
// not tracked by the userland stack, so they report sane fixed values, and
// duplicate is unsupported (the protocol has no handle dup).

use super::TcpStream;
use crate::io;
use crate::net::{Shutdown, SocketAddr};
use crate::sys::net::connection::nonos::transport::unspecified;
use crate::sys::unsupported;
use crate::time::Duration;

impl TcpStream {
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.peer)
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(unspecified())
    }
    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        Ok(())
    }
    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unsupported()
    }
    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn linger(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn nodelay(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
}
