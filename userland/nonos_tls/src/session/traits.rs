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
//! What a session needs from below, and what it reports.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionError {
    /// The client hello could not be built.
    Init,
    /// The socket failed.
    Io,
    /// The server's flight never completed.
    Handshake,
    /// The certificate chain did not verify for this host.
    Certificate,
    /// The server sent more than the caller allows.
    TooLarge,
}

/// The byte stream underneath: a TCP socket, or a buffer in a test.
pub trait Io {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError>;
    /// Read what is available. Zero means nothing arrived this time.
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError>;
}
