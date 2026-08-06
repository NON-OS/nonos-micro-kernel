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
//! Sending on a connected socket.

extern crate alloc;

use alloc::vec::Vec;

use super::super::call::call;
use super::super::constants::OP_SEND;
use super::super::error::SocketError;

/// The request frame is bounded, so a large payload is sent in pieces by the
/// caller rather than silently truncated here.
pub const MAX_SEND: usize = 1024;

pub fn send(port: u32, handle: u32, payload: &[u8]) -> Result<usize, SocketError> {
    let take = payload.len().min(MAX_SEND);
    let mut body = Vec::with_capacity(take + 4);
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&payload[..take]);
    let mut rx = [0u8; 20];
    call(port, OP_SEND, &body, &mut rx)?;
    Ok(take)
}
