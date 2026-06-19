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

pub(super) fn spawn() {
    spawn_l2();
    spawn_ip();
    spawn_udp();
    spawn_dhcp();
    spawn_tcp();
    spawn_dns();
    spawn_nym();
    spawn_sockets();
}

#[cfg(feature = "nonos-capsule-net-l2")]
fn spawn_l2() {
    use crate::userspace::capsule_net_l2 as c;
    super::boot::capsule("NET-L2", "net_l2", c::spawn_net_l2_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-l2"))]
fn spawn_l2() {}

#[cfg(feature = "nonos-capsule-net-ip")]
fn spawn_ip() {
    use crate::userspace::capsule_net_ip as c;
    super::boot::capsule("NET-IP", "net_ip", c::spawn_net_ip_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-ip"))]
fn spawn_ip() {}

#[cfg(feature = "nonos-capsule-net-udp")]
fn spawn_udp() {
    use crate::userspace::capsule_net_udp as c;
    super::boot::capsule("NET-UDP", "net_udp", c::spawn_net_udp_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-udp"))]
fn spawn_udp() {}

#[cfg(feature = "nonos-capsule-net-dhcp")]
fn spawn_dhcp() {
    use crate::userspace::capsule_net_dhcp as c;
    super::boot::capsule("NET-DHCP", "net_dhcp", c::spawn_net_dhcp_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-dhcp"))]
fn spawn_dhcp() {}

#[cfg(feature = "nonos-capsule-net-tcp")]
fn spawn_tcp() {
    use crate::userspace::capsule_net_tcp as c;
    super::boot::capsule("NET-TCP", "net_tcp", c::spawn_net_tcp_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-tcp"))]
fn spawn_tcp() {}

#[cfg(feature = "nonos-capsule-net-dns")]
fn spawn_dns() {
    use crate::userspace::capsule_net_dns as c;
    super::boot::capsule("NET-DNS", "net_dns", c::spawn_net_dns_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-dns"))]
fn spawn_dns() {}

#[cfg(feature = "nonos-capsule-net-sockets")]
fn spawn_sockets() {
    use crate::userspace::capsule_net_sockets as c;
    super::boot::capsule(
        "NET-SOCKETS",
        "net_sockets",
        c::spawn_net_sockets_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-net-sockets"))]
fn spawn_sockets() {}

#[cfg(feature = "nonos-capsule-net-nym")]
fn spawn_nym() {
    use crate::userspace::capsule_net_nym as c;
    super::boot::capsule("NET-NYM", "net_nym", c::spawn_net_nym_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-nym"))]
fn spawn_nym() {}
