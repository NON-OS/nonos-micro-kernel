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

#[derive(Clone, Copy)]
pub struct TcpProbe {
    pub resolve: bool,
    pub socket: bool,
    pub connect: bool,
}

pub fn probe_rpc_tcp(dns_port: u32, sockets_port: u32) -> TcpProbe {
    let Ok(ip) = super::resolve_eth::resolve_eth(dns_port) else {
        return TcpProbe { resolve: false, socket: false, connect: false };
    };
    let Ok(handle) = super::socket_open::socket_open(sockets_port) else {
        return TcpProbe { resolve: true, socket: false, connect: false };
    };
    let connect = super::socket_connect::socket_connect(sockets_port, handle, ip, 443).is_ok();
    let _ = super::socket_close::socket_close(sockets_port, handle);
    TcpProbe { resolve: true, socket: true, connect }
}
