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

//! What a descriptor points at.

#[derive(Clone, Copy, PartialEq, Eq)]
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
    /// A socket net.sockets issued to this capsule.
    Socket,
    /// A Unix socket, both ends inside this capsule.
    Unix,
    /// Anonymous memory with a descriptor, for passing over a socket.
    Memfd,
    /// An interest list a program waits on.
    Epoll,
    /// A timer a program reads or waits on.
    Timer,
    /// One end of a pipe this capsule holds.
    Pipe,
    /// A datagram socket a program opened to talk to a nameserver.
    Resolver,
}
