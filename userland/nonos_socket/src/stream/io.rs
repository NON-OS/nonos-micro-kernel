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
//! Reading and writing a connected stream.

use super::super::error::SocketError;
use super::super::op::{recv, send};
use super::types::TcpStream;

impl TcpStream {
    /// Write everything, in as many sends as the frame size requires.
    pub fn write_all(&mut self, mut data: &[u8]) -> Result<(), SocketError> {
        while !data.is_empty() {
            let sent = send(self.port, self.handle, data)?;
            if sent == 0 {
                return Err(SocketError::Refused);
            }
            data = &data[sent..];
        }
        Ok(())
    }

    /// Read what has arrived. Zero means nothing was ready, not end of stream,
    /// so a caller waiting for a close counts empty reads rather than
    /// stopping at the first one.
    pub fn read(&mut self, into: &mut [u8]) -> Result<usize, SocketError> {
        recv(self.port, self.handle, into)
    }
}
