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

use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::dns;
use smoltcp::wire::{IpAddress, Ipv4Address};

pub fn install_dns_socket(sockets: &mut SocketSet<'static>, dns_ip: [u8; 4]) -> Option<SocketHandle> {
    if dns_ip == [0, 0, 0, 0] {
        return None;
    }
    let server = IpAddress::Ipv4(Ipv4Address(dns_ip));
    let socket = dns::Socket::new(&[server], alloc::vec![]);
    Some(sockets.add(socket))
}
