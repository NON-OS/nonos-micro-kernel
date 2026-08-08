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

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    super::spawn_core::spawn_core();
    super::spawn_legacy_stack::spawn_legacy_stack();
    super::spawn_nym::spawn_nym();
    super::spawn_sockets::spawn_sockets();
    // After net.nym: the SOCKS front end resolves it at startup and would
    // otherwise spin waiting for a service that has not registered yet.
    super::spawn_socks5::spawn_socks5();
}
