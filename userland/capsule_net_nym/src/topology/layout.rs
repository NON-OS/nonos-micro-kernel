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

use alloc::vec::Vec;

use super::types::{TopologyError, DIR_HEADER_LEN, NODE_CAP, NODE_WIRE_LEN};

pub fn check_len(body: &[u8]) -> Result<usize, TopologyError> {
    if body.len() < DIR_HEADER_LEN {
        return Err(TopologyError::BadLength);
    }
    let count = u16::from_le_bytes([body[6], body[7]]) as usize;
    if count == 0 {
        return Err(TopologyError::Empty);
    }
    if count > NODE_CAP {
        return Err(TopologyError::TooLarge);
    }
    let Some(nodes_len) = count.checked_mul(NODE_WIRE_LEN) else {
        return Err(TopologyError::BadLength);
    };
    if body.len() != DIR_HEADER_LEN + nodes_len {
        return Err(TopologyError::BadLength);
    }
    Ok(count)
}

pub fn signed_message(body: &[u8]) -> Vec<u8> {
    let mut msg = Vec::with_capacity(64 + body.len() - DIR_HEADER_LEN);
    msg.extend_from_slice(&body[..64]);
    msg.extend_from_slice(&body[DIR_HEADER_LEN..]);
    msg
}

pub fn u64_at(body: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&body[offset..offset + 8]);
    u64::from_le_bytes(bytes)
}
