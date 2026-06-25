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

use nonos_libc::{mk_debug, mk_service_register};

const NET_TCP: &[u8] = b"net.tcp";
const NET_UDP: &[u8] = b"net.udp";
const NET_DHCP: &[u8] = b"net.dhcp.client";

const PORT_TCP: u32 = 4476;
const PORT_UDP: u32 = 4472;
const PORT_DHCP: u32 = 4474;

pub fn all() {
    let tcp_ok = mk_service_register(NET_TCP.as_ptr(), NET_TCP.len(), PORT_TCP) >= 0;
    let udp_ok = mk_service_register(NET_UDP.as_ptr(), NET_UDP.len(), PORT_UDP) >= 0;
    let dhcp_ok = mk_service_register(NET_DHCP.as_ptr(), NET_DHCP.len(), PORT_DHCP) >= 0;

    if tcp_ok && udp_ok && dhcp_ok {
        let msg = b"[NET-CORE] registered net.tcp net.udp net.dhcp.client\n";
        mk_debug(msg.as_ptr(), msg.len());
    } else {
        let msg = b"[NET-CORE] registration partial failure\n";
        mk_debug(msg.as_ptr(), msg.len());
    }
}
