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
//! Receiving from a connected socket.

extern crate alloc;

use alloc::vec;

use super::super::call::call_t;
use super::super::constants::{HDR_LEN, OP_RECV};
use super::super::error::SocketError;

const RECV_TIMEOUT_MS: u64 = 200;

/// Read what has arrived, up to the length of `out`. Zero means nothing was
/// ready within the timeout, which is not the same as the peer closing.
pub fn recv(port: u32, handle: u32, out: &mut [u8]) -> Result<usize, SocketError> {
    let mut body = [0u8; 4];
    let mut rx = vec![0u8; out.len().saturating_add(HDR_LEN)];
    body.copy_from_slice(&handle.to_le_bytes());
    let n = call_t(port, OP_RECV, &body, &mut rx, RECV_TIMEOUT_MS)?;
    if n < HDR_LEN {
        return Err(SocketError::Protocol);
    }
    // The header states the payload length. Trust the smaller of what it
    // claims, what actually arrived, and what the caller has room for.
    let claimed = u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]]) as usize;
    let take = claimed.min(n - HDR_LEN).min(out.len());
    out[..take].copy_from_slice(&rx[HDR_LEN..HDR_LEN + take]);
    Ok(take)
}
