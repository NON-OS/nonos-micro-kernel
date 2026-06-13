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

use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;

use super::any::SocketAddr;
use super::v4::SocketAddrV4;
use crate::io::{Error, ErrorKind, Result};

pub trait ToSocketAddrs {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>>;
}

impl ToSocketAddrs for SocketAddr {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(vec![*self])
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(vec![SocketAddr::V4(*self)])
    }
}

impl ToSocketAddrs for str {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        if let Ok(v4) = SocketAddrV4::from_str(self) {
            return Ok(vec![SocketAddr::V4(v4)]);
        }
        let (host, port) = self
            .rsplit_once(':')
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "missing port"))?;
        let port: u16 = port.parse().map_err(|_| Error::new(ErrorKind::InvalidInput, "bad port"))?;
        let ip = crate::net::dns::resolve_host(host)?;
        Ok(vec![SocketAddr::V4(SocketAddrV4::new(ip, port))])
    }
}

impl ToSocketAddrs for &str {
    fn to_socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        (**self).to_socket_addrs()
    }
}

pub(crate) fn resolve<A: ToSocketAddrs>(addr: A) -> Result<([u8; 4], u16)> {
    let addrs = addr.to_socket_addrs()?;
    let first =
        addrs.into_iter().next().ok_or_else(|| Error::new(ErrorKind::InvalidInput, "no address"))?;
    Ok(first.v4_parts())
}
