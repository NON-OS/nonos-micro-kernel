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

use super::constants::{OP_SOCKET, SOCKETS_MAGIC, SOCKET_FAMILY_IP4, SOCKET_KIND_STREAM};

pub fn socket_open(sockets_port: u32) -> Result<u32, ()> {
    if super::mixnet::is_on() {
        return super::mixnet::open();
    }
    let mut body = [0u8; 4];
    let mut rx = [0u8; 32];
    body[0..2].copy_from_slice(&SOCKET_FAMILY_IP4.to_le_bytes());
    body[2..4].copy_from_slice(&SOCKET_KIND_STREAM.to_le_bytes());
    let n = super::call::call(sockets_port, SOCKETS_MAGIC, OP_SOCKET, &body, &mut rx)?;
    if n < 24 {
        return Err(());
    }
    Ok(u32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]))
}
