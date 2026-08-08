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
//! Connecting to a named host.

extern crate alloc;

use alloc::vec;

use super::super::call::call_t;
use super::super::constants::OP_CONNECT_HOST;
use super::super::error::SocketError;

/// Resolution and connection both happen in the sockets capsule, so a caller
/// never handles an address and cannot be pointed at one by a reply it
/// misparsed.
const CONNECT_TIMEOUT_MS: u64 = 9000;

pub fn connect_host(
    port: u32,
    handle: u32,
    host: &str,
    remote_port: u16,
) -> Result<(), SocketError> {
    let h = host.as_bytes();
    // The longest legal domain name is 253 bytes.
    if h.is_empty() || h.len() > 253 {
        return Err(SocketError::BadHost);
    }
    let mut body = vec![0u8; 8 + h.len()];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..6].copy_from_slice(&remote_port.to_le_bytes());
    body[6..8].copy_from_slice(&(h.len() as u16).to_le_bytes());
    body[8..].copy_from_slice(h);
    let mut rx = [0u8; 20];
    call_t(port, OP_CONNECT_HOST, &body, &mut rx, CONNECT_TIMEOUT_MS)?;
    Ok(())
}
