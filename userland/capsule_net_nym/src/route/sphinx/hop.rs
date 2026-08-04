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

extern crate alloc;

use alloc::vec::Vec;

use super::types::HOP_BYTES;
use crate::crypto::hmac_sha256;
use crate::packet::PacketError;
use crate::topology::{Node, Role};

pub fn write(
    block: &mut [u8],
    id: u32,
    flags: u8,
    idx: u8,
    hop: &Node,
    key: &[u8; 32],
) -> Result<(), PacketError> {
    if block.len() != HOP_BYTES {
        return Err(PacketError::BadLength);
    }
    let mac = mac(id, flags, idx, hop, key)?;
    block[..32].copy_from_slice(&mac);
    block[32..34].copy_from_slice(&hop.delay_ms.to_le_bytes());
    block[34..38].copy_from_slice(&hop.ip);
    block[38..40].copy_from_slice(&hop.port.to_le_bytes());
    block[40] = role_id(hop.role);
    block[41] = hop.layer;
    block[42..55].copy_from_slice(&hop.identity[..13]);
    Ok(())
}

fn mac(id: u32, flags: u8, idx: u8, hop: &Node, key: &[u8; 32]) -> Result<[u8; 32], PacketError> {
    let mut data = Vec::with_capacity(78);
    data.extend_from_slice(&id.to_le_bytes());
    data.push(flags);
    data.push(idx);
    data.extend_from_slice(&hop.identity);
    data.extend_from_slice(&hop.ip);
    data.extend_from_slice(&hop.port.to_le_bytes());
    data.extend_from_slice(&hop.delay_ms.to_le_bytes());
    let mut out = [0u8; 32];
    hmac_sha256(key, &data, &mut out).map_err(|_| PacketError::Crypto)?;
    Ok(out)
}

fn role_id(role: Role) -> u8 {
    match role {
        Role::EntryGateway => 1,
        Role::Mix => 2,
        Role::ExitGateway => 3,
    }
}
