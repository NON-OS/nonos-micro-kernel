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

use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::dhcpv4;

pub fn create(sockets: &mut SocketSet<'static>) -> SocketHandle {
    sockets.add(dhcpv4::Socket::new())
}

pub fn drain_events(handle: SocketHandle, sockets: &mut SocketSet<'static>) {
    sockets.get_mut::<dhcpv4::Socket>(handle).poll();
}
