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

use alloc::vec;

use super::constants::{OP_CONNECT_HOST, SOCKETS_MAGIC};

const CONNECT_TIMEOUT_MS: u64 = 9000;

pub fn socket_connect_host(
    sockets_port: u32,
    handle: u32,
    host: &str,
    port: u16,
) -> Result<(), ()> {
    if super::mixnet::is_on() {
        return super::mixnet::connect();
    }
    let h = host.as_bytes();
    if h.is_empty() || h.len() > 253 {
        return Err(());
    }
    let mut body = vec![0u8; 8 + h.len()];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..6].copy_from_slice(&port.to_le_bytes());
    body[6..8].copy_from_slice(&(h.len() as u16).to_le_bytes());
    body[8..].copy_from_slice(h);
    let mut rx = [0u8; 20];
    super::call::call_t(
        sockets_port,
        SOCKETS_MAGIC,
        OP_CONNECT_HOST,
        &body,
        &mut rx,
        CONNECT_TIMEOUT_MS,
    )?;
    Ok(())
}
