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

//! Bytes on a socket this capsule opened for itself.

use alloc::vec::Vec;

use super::call::call;
use super::ops::{OP_CLOSE, OP_RECV, OP_SEND};

/// One transfer. The service caps a reply, so a body arrives in pieces.
const CHUNK: usize = 32 << 10;

pub fn send_all(handle: u32, bytes: &[u8]) -> Option<()> {
    for part in bytes.chunks(CHUNK) {
        let mut body = Vec::with_capacity(4 + part.len());
        body.extend_from_slice(&handle.to_le_bytes());
        body.extend_from_slice(part);
        match call(OP_SEND, &body, 0) {
            Some((0, _)) => {}
            _ => return None,
        }
    }
    Some(())
}

/// Read until the peer closes, which is what Connection: close gives.
pub fn recv_all(handle: u32, limit: usize) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    loop {
        match call(OP_RECV, &handle.to_le_bytes(), CHUNK) {
            Some((0, part)) if part.is_empty() => return Some(out),
            Some((0, part)) => out.extend_from_slice(&part),
            _ => return Some(out),
        }
        if out.len() > limit {
            return None;
        }
    }
}

pub fn close(handle: u32) {
    let _ = call(OP_CLOSE, &handle.to_le_bytes(), 0);
}
