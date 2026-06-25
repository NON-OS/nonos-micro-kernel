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

use spin::Mutex;
use smoltcp::iface::{Interface, SocketHandle, SocketSet};
use crate::device::NicDevice;

pub struct NetState {
    pub iface: Interface,
    pub sockets: SocketSet<'static>,
    pub device: NicDevice,
    pub dhcp_handle: SocketHandle,
}

// SAFETY: The capsule has a single execution context (no threads); spin::Mutex
// is the exclusive access gate; Interface, SocketSet<'static>, and NicDevice
// are themselves Send.
unsafe impl Send for NetState {}

pub static NET: Mutex<Option<NetState>> = Mutex::new(None);

pub fn store(state: NetState) {
    *NET.lock() = Some(state);
}

pub fn with_iface<R>(
    f: impl FnOnce(&mut Interface, &mut SocketSet<'static>, &mut NicDevice) -> R,
) -> Option<R> {
    let mut guard = NET.lock();
    let state = guard.as_mut()?;
    Some(f(&mut state.iface, &mut state.sockets, &mut state.device))
}
