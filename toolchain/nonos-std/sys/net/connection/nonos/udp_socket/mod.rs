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

// A UDP socket over net.sockets. Holds the socket handle and the bound local
// address. The operations are split by concern into the sibling bind, io and
// options modules.

mod bind;
mod io;
mod options;

use crate::fmt;
use crate::net::SocketAddr;
use crate::sys::net::connection::nonos::transport::close;

pub struct UdpSocket {
    pub(super) handle: u32,
    pub(super) local: SocketAddr,
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        close(self.handle);
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UdpSocket").field("local", &self.local).finish()
    }
}
