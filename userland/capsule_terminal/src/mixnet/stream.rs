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

use nonos_libc::mk_service_lookup;
use nonos_socket::TcpStream;

use super::socks::SocksStream;

const PROXY: &[u8] = b"net.socks5";

/// How the terminal reaches a host.
///
/// Presence of the proxy decides it, the same rule the browser follows. A
/// request that would otherwise leave directly names this machine to the
/// host, and nothing on the command line would show that it had.
pub enum Wire {
    Mixnet(SocksStream),
    Direct(TcpStream),
}

impl Wire {
    pub fn connect(host: &str, port: u16) -> Result<Self, ()> {
        if let Some(proxy) = proxy_port() {
            // No fallback: reverting to a direct connection here would turn
            // one failed request into a disclosure.
            return SocksStream::connect(proxy, host, port).map(Wire::Mixnet);
        }
        TcpStream::connect(host, port).map(Wire::Direct).map_err(|_| ())
    }

    pub fn write_all(&mut self, data: &[u8]) -> Result<(), ()> {
        match self {
            Wire::Mixnet(s) => s.write_all(data),
            Wire::Direct(s) => s.write_all(data).map_err(|_| ()),
        }
    }

    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, ()> {
        match self {
            Wire::Mixnet(s) => s.read(into),
            Wire::Direct(s) => s.read(into).map_err(|_| ()),
        }
    }

    pub fn is_mixnet(&self) -> bool {
        matches!(self, Wire::Mixnet(_))
    }
}

/// Whether the proxy capsule is reachable.
pub fn proxy_available() -> bool {
    proxy_port().is_some()
}

fn proxy_port() -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(PROXY.as_ptr(), PROXY.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return None;
    }
    Some(port)
}
