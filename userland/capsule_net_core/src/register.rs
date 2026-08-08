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
const NET_DNS: &[u8] = b"net.dns";
const NET_IP: &[u8] = b"net.ip";

const PORT_TCP: u32 = 4476;
const PORT_UDP: u32 = 4472;
const PORT_DHCP: u32 = 4474;
const PORT_DNS: u32 = 4478;
const PORT_IP: u32 = 4479;

fn one(name: &[u8], port: u32, label: &[u8]) -> bool {
    if mk_service_register(name.as_ptr(), name.len(), port) >= 0 {
        return true;
    }
    // Name the failed endpoint so a registration regression is diagnosable
    // from the serial log instead of an opaque "partial failure".
    mk_debug(label.as_ptr(), label.len());
    false
}

pub fn all() {
    let tcp_ok = one(NET_TCP, PORT_TCP, b"[NET-CORE] register FAILED net.tcp\n");
    let udp_ok = one(NET_UDP, PORT_UDP, b"[NET-CORE] register FAILED net.udp\n");
    let dhcp_ok = one(NET_DHCP, PORT_DHCP, b"[NET-CORE] register FAILED net.dhcp.client\n");
    let dns_ok = one(NET_DNS, PORT_DNS, b"[NET-CORE] register FAILED net.dns\n");
    let ip_ok = one(NET_IP, PORT_IP, b"[NET-CORE] register FAILED net.ip\n");

    if tcp_ok && udp_ok && dhcp_ok && dns_ok && ip_ok {
        let msg = b"[NET-CORE] registered net.tcp net.udp net.dhcp.client net.dns net.ip\n";
        mk_debug(msg.as_ptr(), msg.len());
    } else {
        let msg = b"[NET-CORE] registration partial failure\n";
        mk_debug(msg.as_ptr(), msg.len());
    }
}
