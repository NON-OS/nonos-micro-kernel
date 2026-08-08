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
//! Opening a connection.

use super::super::error::SocketError;
use super::super::lookup::lookup;
use super::super::op::{close, connect_host, open};
use super::types::TcpStream;

impl TcpStream {
    /// Connect to `host` on `port` through the sockets capsule.
    pub fn connect(host: &str, port: u16) -> Result<TcpStream, SocketError> {
        let service = lookup(b"net.sockets");
        if service == 0 {
            return Err(SocketError::NoService);
        }
        let handle = open(service)?;
        if connect_host(service, handle, host, port).is_err() {
            // The handle exists even though the connection failed, so it has
            // to be released here rather than left to the capsule.
            close(service, handle);
            return Err(SocketError::Refused);
        }
        Ok(TcpStream { port: service, handle })
    }
}
