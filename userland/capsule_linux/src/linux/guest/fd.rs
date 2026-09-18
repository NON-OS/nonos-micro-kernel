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


//! A guest's file descriptors. A descriptor is a number the guest chose to
//! believe in; what it points at is this personality's business, and is
//! never a NONOS handle the guest could name on its own.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;

#[derive(PartialEq, Eq)]
pub enum Kind {
    /// Closed, and reusable.
    Free,
    /// The guest's own console, carried to the host's output.
    Stdin,
    Stdout,
    Stderr,
    /// A file in the store, held open on the server.
    File,
    /// A directory, listed once when it was opened.
    Dir,
}

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
