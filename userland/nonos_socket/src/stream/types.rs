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
//! What a stream owns.

/// A connected socket, closed when it goes out of scope.
///
/// Owning the handle is what stops a socket outliving the code that opened
/// it. An early return on a failed handshake would otherwise leave the
/// capsule holding a connection nobody will ever read or close.
pub struct TcpStream {
    pub(super) port: u32,
    pub(super) handle: u32,
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        super::super::op::close(self.port, self.handle);
    }
}
