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

// The descriptor-owning core of the socket types, the shape `os::fd` reaches
// through: `TcpStream`, `TcpListener` and `UdpSocket` each hold a `Socket`,
// and a `Socket` holds the `FileDesc` whose table slot maps back to the
// net.sockets handle. Closing is RAII through the descriptor; the handle
// itself is the table's single source of truth.

use super::transport::{close, err};
use crate::io;
use crate::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, RawFd};
use crate::sys::fd::{Backing, FileDesc, get, install};
use crate::sys::{FromInner, IntoInner};

pub struct Socket(FileDesc);

impl Socket {
    // Take ownership of a connected net.sockets handle by registering it in
    // the descriptor table. On a full table the handle is closed before the
    // error returns, so it cannot leak on the service side.
    pub(crate) fn register(handle: u32) -> io::Result<Socket> {
        match install(Backing::Socket { handle }) {
            // SAFETY: the slot was just installed and is owned by no other
            // value, so the new descriptor has sole ownership.
            Ok(fd) => Ok(Socket(unsafe { FileDesc::from_raw_fd(fd) })),
            Err(e) => {
                close(handle);
                Err(e)
            }
        }
    }

    // The net.sockets handle behind this socket. Looked up per call so a
    // socket reconstructed from a bare descriptor and one made by connect
    // resolve through the same table truth.
    pub(crate) fn handle(&self) -> io::Result<u32> {
        match get(self.0.as_raw_fd()) {
            Some(Backing::Socket { handle }) => Ok(handle),
            _ => Err(err("descriptor does not name a live socket")),
        }
    }
}

impl AsFd for Socket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for Socket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl FromInner<FileDesc> for Socket {
    fn from_inner(desc: FileDesc) -> Socket {
        Socket(desc)
    }
}

impl IntoInner<FileDesc> for Socket {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}
