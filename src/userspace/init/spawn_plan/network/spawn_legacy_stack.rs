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

#[cfg(not(feature = "nonos-capsule-net-core"))]
pub(super) fn spawn_legacy_stack() {
    super::spawn_l2::spawn_l2();
    super::spawn_ip::spawn_ip();
    super::spawn_udp::spawn_udp();
    super::spawn_dhcp::spawn_dhcp();
    super::spawn_tcp::spawn_tcp();
    super::spawn_dns::spawn_dns();
    super::spawn_ntp::spawn_ntp();
}

#[cfg(feature = "nonos-capsule-net-core")]
pub(super) fn spawn_legacy_stack() {}
