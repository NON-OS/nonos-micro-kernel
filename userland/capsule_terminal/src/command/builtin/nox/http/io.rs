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

use nonos_http::{HttpError, Stream};
use nonos_socket::TcpStream;
use nonos_tls::{Io, SessionError};

/// The socket seen as a byte stream. TLS reads and writes it directly for
/// `https`; plain `http` drives it through the same type so both paths share
/// one connection implementation.
pub struct SocketIo {
    pub stream: TcpStream,
}

impl Io for SocketIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.stream.write_all(data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        self.stream.read(into).map_err(|_| SessionError::Io)
    }
}

impl Stream for SocketIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), HttpError> {
        self.stream.write_all(data).map_err(|_| HttpError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, HttpError> {
        self.stream.read(into).map_err(|_| HttpError::Io)
    }
}
