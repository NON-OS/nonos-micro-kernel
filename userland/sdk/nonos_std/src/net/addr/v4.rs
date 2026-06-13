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

use core::fmt;
use core::str::FromStr;

use super::ip::Ipv4Addr;
use super::parse::invalid;
use crate::io::Error;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SocketAddrV4 {
    ip: Ipv4Addr,
    port: u16,
}

impl SocketAddrV4 {
    pub const fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self { ip, port }
    }

    pub const fn ip(&self) -> &Ipv4Addr {
        &self.ip
    }

    pub const fn port(&self) -> u16 {
        self.port
    }
}

impl fmt::Display for SocketAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl FromStr for SocketAddrV4 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let (host, port) = s.rsplit_once(':').ok_or_else(invalid)?;
        let ip = host.parse::<Ipv4Addr>()?;
        let port = port.parse::<u16>().map_err(|_| invalid())?;
        Ok(Self { ip, port })
    }
}
