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

// UDP socket options and address queries. The userland stack is IPv4 and
// tracks none of these per socket, so timeouts, broadcast, ttl and the
// nonblocking flag report fixed values, multicast joins/leaves are
// unsupported, and duplicate has no handle-dup form.

use super::UdpSocket;
use crate::io;
use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use crate::sys::net::connection::nonos::transport::unspecified;
use crate::sys::unsupported;
use crate::time::Duration;

impl UdpSocket {
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(unspecified())
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local)
    }
    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unsupported()
    }
    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn broadcast(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        Ok(1)
    }
    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }
    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }
    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }
    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
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
    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
}
