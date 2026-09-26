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

//! Sockets for this capsule's own use, rather than a guest's.

use alloc::vec::Vec;

use super::call::call;
use super::ops::{DOMAIN, KIND_MIXNET, OP_CONNECT_HOST, OP_SOCKET};

/// Over the mixnet, like everything else.
pub fn open_stream() -> Option<u32> {
    let mut body = Vec::with_capacity(4);
    body.extend_from_slice(&DOMAIN.to_le_bytes());
    body.extend_from_slice(&KIND_MIXNET.to_le_bytes());
    match call(OP_SOCKET, &body, 8) {
        Some((0, out)) if out.len() >= 4 => {
            Some(u32::from_le_bytes([out[0], out[1], out[2], out[3]]))
        }
        _ => None,
    }
}

pub fn connect_host(handle: u32, host: &str, port: u16) -> Option<()> {
    let mut body = Vec::with_capacity(7 + host.len());
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&port.to_le_bytes());
    body.push(host.len() as u8);
    body.extend_from_slice(host.as_bytes());
    match call(OP_CONNECT_HOST, &body, 0) {
        Some((0, _)) => Some(()),
        _ => None,
    }
}
