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
//! Opening a stream socket.

use super::super::call::call;
use super::super::constants::{OP_SOCKET, SOCKET_FAMILY_IP4, SOCKET_KIND_STREAM};
use super::super::error::SocketError;

/// Allocate an IPv4 stream socket and return its handle.
pub fn open(port: u32) -> Result<u32, SocketError> {
    let mut body = [0u8; 4];
    let mut rx = [0u8; 32];
    body[0..2].copy_from_slice(&SOCKET_FAMILY_IP4.to_le_bytes());
    body[2..4].copy_from_slice(&SOCKET_KIND_STREAM.to_le_bytes());
    let n = call(port, OP_SOCKET, &body, &mut rx)?;
    if n < 24 {
        return Err(SocketError::Protocol);
    }
    Ok(u32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]))
}
