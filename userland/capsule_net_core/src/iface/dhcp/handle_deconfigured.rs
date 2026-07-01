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

use crate::state;

pub fn handle_deconfigured(
    iface: &mut Interface,
    sockets: &mut SocketSet<'static>,
    dns_slot: &mut Option<SocketHandle>,
) {
    iface.update_ip_addrs(|addrs| addrs.clear());
    let _ = iface.routes_mut().remove_default_ipv4_route();
    if let Some(old) = dns_slot.take() {
        sockets.remove(old);
    }
    state::set_lease(None);
}
