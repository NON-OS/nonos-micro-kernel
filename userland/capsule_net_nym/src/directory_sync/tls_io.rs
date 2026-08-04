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

use nonos_tls::{Io, SessionError};

use crate::tcp_client;

/// The gateway transport, seen as the byte stream a TLS session drives.
///
/// The directory is fetched over the same `net.tcp` client that reaches
/// gateways, which is already proven against real peers. Nothing about the
/// fetch is anonymous: it happens before there is a mixnet to be anonymous
/// over, which is the bootstrap problem every mixnet client has.
pub struct TcpIo {
    pub tcp_port: u32,
    pub stream: u32,
}

impl Io for TcpIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        tcp_client::send_all(self.tcp_port, self.stream, data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        tcp_client::recv(self.tcp_port, self.stream, into).map_err(|_| SessionError::Io)
    }
}
