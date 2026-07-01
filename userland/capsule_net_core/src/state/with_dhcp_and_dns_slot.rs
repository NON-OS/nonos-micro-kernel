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

use crate::state::globals::NET;

pub fn with_dhcp_and_dns_slot<R>(
    f: impl FnOnce(&mut Interface, &mut SocketSet<'static>, SocketHandle, &mut Option<SocketHandle>) -> R,
) -> Option<R> {
    let mut guard = NET.lock();
    let state = guard.as_mut()?;
    Some(f(&mut state.iface, &mut state.sockets, state.dhcp_handle, &mut state.dns_handle))
}
