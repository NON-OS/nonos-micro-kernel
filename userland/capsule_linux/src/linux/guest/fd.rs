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

//! A guest's file descriptors.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;

pub use super::fd_kind::Kind;

pub struct Fd {
    pub kind: Kind,
    /// Byte offset for the kinds that have one.
    pub offset: u64,
    /// The size the server reported when this was opened.
    pub size: u64,
    /// The path the guest named, kept for stat and for the flush on close.
    pub path: Vec<u8>,
    /// The server-side handle, for a file opened to read.
    pub stream: Option<VfsStream>,
    /// Bytes written by the guest and not yet on the server.
    pub pending: Vec<u8>,
    /// The entries of a directory, taken once at open.
    pub names: Vec<String>,
    /// Set when the guest asked to write, so close knows to flush.
    pub writable: bool,
    /// The net.sockets handle behind a socket descriptor.
    pub handle: u32,
    /// An epoll interest list: descriptor, events, and the token the
    /// program gets back, which is its own and never interpreted.
    pub watch: Vec<(u64, u32, u64)>,
    /// When a timer next fires, in milliseconds of uptime.
    pub expiry: u64,
    /// Datagrams waiting to be read, oldest first, each with the address it
    /// should appear to come from.
    pub replies: Vec<(Vec<u8>, [u8; 6])>,
    /// Closed by exec rather than carried into the new program. A shell
    /// leaves its own descriptors set this way before it runs a command.
    pub cloexec: bool,
}

impl Fd {
    /// The three a program is entitled to assume are already open.
    pub fn standard() -> Vec<Fd> {
        vec![Fd::console(Kind::Stdin), Fd::console(Kind::Stdout), Fd::console(Kind::Stderr)]
    }

    pub fn is_open(&self) -> bool {
        self.kind != Kind::Free
    }
}
