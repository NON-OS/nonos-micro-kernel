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

use smoltcp::iface::SocketHandle;
use smoltcp::socket::icmp;
use spin::Mutex;

use crate::iface::icmp_socket::install_icmp_socket;
use crate::state;

/// The ICMP socket, and the identifier it is bound to.
///
/// One socket rather than one per caller: an identifier is how ICMP tells
/// conversations apart, so a second caller with its own identifier rebinds
/// rather than getting a socket of its own.
static ICMP: Mutex<Option<(SocketHandle, u16)>> = Mutex::new(None);

/// Run `f` against the ICMP socket bound to `ident`, creating or rebinding it
/// first. `None` when the interface is not up yet.
pub fn with_icmp<R>(ident: u16, f: impl FnOnce(&mut icmp::Socket<'static>) -> R) -> Option<R> {
    let mut slot = ICMP.lock();
    state::with_iface(|_iface, sockets, _dev| {
        let handle = match *slot {
            Some((handle, bound)) if bound == ident => handle,
            Some((handle, _)) => {
                let sock = sockets.get_mut::<icmp::Socket>(handle);
                let _ = sock.bind(icmp::Endpoint::Ident(ident));
                *slot = Some((handle, ident));
                handle
            }
            None => {
                let handle = install_icmp_socket(sockets, ident);
                *slot = Some((handle, ident));
                handle
            }
        };
        f(sockets.get_mut::<icmp::Socket>(handle))
    })
}
