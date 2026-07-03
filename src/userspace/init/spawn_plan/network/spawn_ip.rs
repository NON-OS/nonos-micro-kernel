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

#[cfg(feature = "nonos-capsule-net-ip")]
pub(super) fn spawn_ip() {
    use crate::userspace::capsule_net_ip as c;
    super::super::boot::capsule("NET-IP", "net_ip", c::spawn_net_ip_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-net-ip"))]
pub(super) fn spawn_ip() {}
