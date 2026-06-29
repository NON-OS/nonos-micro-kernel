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

use super::constants::{OP_CONNECT, SOCKETS_MAGIC};

const CONNECT_TIMEOUT_MS: u64 = 9000;

pub fn socket_connect(sockets_port: u32, handle: u32, ip: [u8; 4], port: u16) -> Result<(), ()> {
    let mut body = [0u8; 10];
    let mut rx = [0u8; 20];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(&ip);
    body[8..10].copy_from_slice(&port.to_le_bytes());
    super::call::call_t(sockets_port, SOCKETS_MAGIC, OP_CONNECT, &body, &mut rx, CONNECT_TIMEOUT_MS)?;
    Ok(())
}
