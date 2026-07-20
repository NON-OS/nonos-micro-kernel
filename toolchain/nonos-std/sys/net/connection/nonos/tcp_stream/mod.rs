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

// A connected TCP stream over net.sockets. Holds its handle through a
// `Socket` descriptor (so close is RAII in the fd table and `os::fd` can
// borrow or take it), the peer address, and the socket options (read/write
// timeouts and nonblocking mode) so a getter returns what its setter stored.
// The operations are split by concern into the sibling connect, io, timeout,
// and options modules.

mod connect;
mod io;
mod options;
mod timeout;

use core::sync::atomic::{AtomicBool, AtomicU64};

use super::socket::Socket;
use super::transport::unspecified;
use crate::fmt;
use crate::net::SocketAddr;
use crate::sys::FromInner;

pub struct TcpStream {
    pub(super) socket: Socket,
    pub(super) peer: SocketAddr,
    // Socket options live here so a getter returns what its setter stored. A
    // 0 ms timeout means "no deadline"; a nonblocking read/write uses a 1 ms
    // deadline and maps a lapse to WouldBlock.
    pub(super) read_timeout_ms: AtomicU64,
    pub(super) write_timeout_ms: AtomicU64,
    pub(super) nonblocking: AtomicBool,
}

impl TcpStream {
    pub(crate) fn wrap(handle: u32, peer: SocketAddr) -> crate::io::Result<TcpStream> {
        Ok(TcpStream {
            socket: Socket::register(handle)?,
            peer,
            read_timeout_ms: AtomicU64::new(0),
            write_timeout_ms: AtomicU64::new(0),
            nonblocking: AtomicBool::new(false),
        })
    }

    pub(crate) fn socket(&self) -> &Socket {
        &self.socket
    }

    pub(crate) fn into_socket(self) -> Socket {
        self.socket
    }
}

impl FromInner<Socket> for TcpStream {
    // Reconstruction from a bare descriptor (the `From<OwnedFd>` path). The
    // peer address is not recoverable from the table, so it reads as
    // unspecified, the same degradation an accepted stream reports.
    fn from_inner(socket: Socket) -> TcpStream {
        TcpStream {
            socket,
            peer: unspecified(),
            read_timeout_ms: AtomicU64::new(0),
            write_timeout_ms: AtomicU64::new(0),
            nonblocking: AtomicBool::new(false),
        }
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream").field("peer", &self.peer).finish()
    }
}
