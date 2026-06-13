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

use super::constants::{
    DNS_MAGIC, NYM_MAGIC, SERVICE_DNS, SERVICE_NYM, SERVICE_SOCKETS, SOCKETS_MAGIC,
};
use super::status::NetStatus;

pub fn probe_network() -> NetStatus {
    let dns_port = super::lookup::lookup(SERVICE_DNS);
    let sockets_port = super::lookup::lookup(SERVICE_SOCKETS);
    let nym_port = super::lookup::lookup(SERVICE_NYM);
    let dns_ok = dns_port != 0 && super::health::health(dns_port, DNS_MAGIC);
    let sockets_ok = sockets_port != 0 && super::health::health(sockets_port, SOCKETS_MAGIC);
    let nym_ok = nym_port != 0 && super::health::health(nym_port, NYM_MAGIC);
    let route_ready = dns_ok && sockets_ok && nym_ok;
    let rpc_tcp_ok = dns_ok && sockets_ok && super::probe_rpc_tcp::probe_rpc_tcp(dns_port, sockets_port);
    let rpc_codec_ok = super::super::rpc::self_check();
    NetStatus {
        dns_ok,
        sockets_ok,
        nym_ok,
        route_ready,
        rpc_tcp_ok,
        rpc_codec_ok,
        status: if rpc_tcp_ok { b"rpc tcp ready" } else if route_ready { b"route ready" } else { b"route blocked" },
    }
}
