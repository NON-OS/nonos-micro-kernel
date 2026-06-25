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

use nonos_libc::mk_debug;
use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::dhcpv4;
use smoltcp::wire::IpCidr;

use crate::state::{self, Lease};

pub fn create(sockets: &mut SocketSet<'static>) -> SocketHandle {
    sockets.add(dhcpv4::Socket::new())
}

pub fn poll_event() {
    state::with_dhcp(|iface, sockets, handle| {
        let event = sockets.get_mut::<dhcpv4::Socket>(handle).poll();
        match event {
            Some(dhcpv4::Event::Configured(cfg)) => {
                iface.update_ip_addrs(|addrs| {
                    addrs.clear();
                    let _ = addrs.push(IpCidr::Ipv4(cfg.address));
                });
                if let Some(r) = cfg.router {
                    let _ = iface.routes_mut().add_default_ipv4_route(r);
                }
                let ip = cfg.address.address().0;
                let prefix = cfg.address.prefix_len();
                let gw = cfg.router.map(|r| r.0).unwrap_or([0u8; 4]);
                let dns = cfg.dns_servers.first().map(|d| d.0).unwrap_or([0u8; 4]);
                emit_lease_marker(ip, prefix, gw);
                state::set_lease(Some(Lease { ip, prefix, gw, dns, secs: 0, bound: true }));
            }
            Some(dhcpv4::Event::Deconfigured) => {
                iface.update_ip_addrs(|addrs| addrs.clear());
                let _ = iface.routes_mut().remove_default_ipv4_route();
                state::set_lease(None);
            }
            None => {}
        }
    });
}

fn emit_lease_marker(ip: [u8; 4], prefix: u8, gw: [u8; 4]) {
    let mut buf = [0u8; 64];
    let msg = b"[NET-CORE] lease ";
    let n = fill_marker(&mut buf, msg, ip, prefix, gw);
    mk_debug(buf.as_ptr(), n);
}

fn fill_marker(buf: &mut [u8; 64], prefix_msg: &[u8], ip: [u8; 4], prefix: u8, gw: [u8; 4]) -> usize {
    let mut pos = 0usize;
    for &b in prefix_msg {
        buf[pos] = b; pos += 1;
    }
    pos = write_octet_quad(buf, pos, ip);
    buf[pos] = b'/'; pos += 1;
    pos = write_decimal_u8(buf, pos, prefix);
    buf[pos] = b' '; pos += 1;
    for &b in b"gw " { buf[pos] = b; pos += 1; }
    pos = write_octet_quad(buf, pos, gw);
    buf[pos] = b'\n'; pos += 1;
    pos
}

fn write_octet_quad(buf: &mut [u8; 64], mut pos: usize, quad: [u8; 4]) -> usize {
    for (i, &byte) in quad.iter().enumerate() {
        if i > 0 { buf[pos] = b'.'; pos += 1; }
        pos = write_decimal_u8(buf, pos, byte);
    }
    pos
}

fn write_decimal_u8(buf: &mut [u8; 64], mut pos: usize, val: u8) -> usize {
    if val >= 100 { buf[pos] = b'0' + val / 100; pos += 1; }
    if val >= 10  { buf[pos] = b'0' + (val / 10) % 10; pos += 1; }
    buf[pos] = b'0' + val % 10; pos += 1;
    pos
}
