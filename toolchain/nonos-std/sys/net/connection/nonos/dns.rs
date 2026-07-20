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

// Hostname resolution over the net.dns service. A literal IPv4 address is
// used as-is; otherwise the name is sent to net.dns, which replies with four
// address octets. The userland resolver returns a single A record, so
// LookupHost yields one address at the requested port.

use crate::io;
use crate::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use crate::sys::net::connection::nonos::transport::{BODY, DNS_MAGIC, DNS_NAME, err, ipc};
use crate::vec::Vec;

pub struct LookupHost {
    it: crate::vec::IntoIter<SocketAddr>,
    port: u16,
}

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.it.next()
    }
}

pub fn lookup_host(host: &str, port: u16) -> io::Result<LookupHost> {
    let ip = if let Ok(v4) = host.parse::<Ipv4Addr>() {
        v4
    } else {
        if host.is_empty() || host.len() > 255 {
            return Err(err("bad hostname"));
        }
        let rx = ipc(DNS_NAME, DNS_MAGIC, 2, host.as_bytes(), 4, 0)?;
        let b = &rx[BODY..];
        if b.len() < 4 {
            return Err(err("dns short reply"));
        }
        Ipv4Addr::new(b[0], b[1], b[2], b[3])
    };
    let mut v: Vec<SocketAddr> = Vec::with_capacity(1);
    v.push(SocketAddr::V4(SocketAddrV4::new(ip, port)));
    Ok(LookupHost { it: v.into_iter(), port })
}
