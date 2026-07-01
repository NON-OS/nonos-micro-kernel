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

use smoltcp::iface::{Interface, SocketHandle, SocketSet};
use smoltcp::wire::IpCidr;

use crate::iface::dhcp::{emit_lease_marker, emit_status_selfcheck, install_dns_socket};
use crate::iface::dhcp::types::ConfiguredLease;
use crate::state::{self, Lease};

pub fn handle_configured(
    iface: &mut Interface,
    sockets: &mut SocketSet<'static>,
    dns_slot: &mut Option<SocketHandle>,
    cfg: ConfiguredLease,
) {
    iface.update_ip_addrs(|addrs| {
        addrs.clear();
        let _ = addrs.push(IpCidr::Ipv4(cfg.address));
    });
    if let Some(r) = cfg.router {
        let _ = iface.routes_mut().add_default_ipv4_route(r);
    }
    if let Some(old) = dns_slot.take() {
        sockets.remove(old);
    }
    let ip = cfg.address.address().0;
    let prefix = cfg.address.prefix_len();
    let gw = match cfg.router {
        Some(r) => r.0,
        None => [0u8; 4],
    };
    let dns = cfg.dns;
    emit_lease_marker::emit_lease_marker(ip, prefix, gw);
    state::set_lease(Some(Lease { ip, prefix, gw, dns, secs: 0, bound: true }));
    *dns_slot = install_dns_socket::install_dns_socket(sockets, dns);
    emit_status_selfcheck::emit_status_selfcheck();
}
