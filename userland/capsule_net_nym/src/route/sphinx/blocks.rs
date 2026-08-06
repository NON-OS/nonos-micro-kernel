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

use super::types::{HOP_BYTES, PREFIX_LEN, ROUTE_HEADER_LEN};
use super::{hop, key};
use crate::crypto::x25519_shared;
use crate::packet::PacketError;
use crate::topology::{Node, ROUTE_HOPS};

pub fn write(
    out: &mut [u8; ROUTE_HEADER_LEN],
    id: u32,
    flags: u8,
    seed: &[u8; 32],
    cred: &[u8; 32],
    hops: &[Node; ROUTE_HOPS],
    private: &[u8; 32],
) -> Result<[[u8; 32]; ROUTE_HOPS], PacketError> {
    let mut keys = [[0u8; 32]; ROUTE_HOPS];
    for idx in 0..ROUTE_HOPS {
        let mut shared = [0u8; 32];
        x25519_shared(private, &hops[idx].packet_key, &mut shared)
            .map_err(|_| PacketError::Crypto)?;
        keys[idx] = key::hop_key(seed, idx as u8, cred, &shared)?;
        let start = PREFIX_LEN + idx * HOP_BYTES;
        hop::write(
            &mut out[start..start + HOP_BYTES],
            id,
            flags,
            idx as u8,
            &hops[idx],
            &keys[idx],
        )?;
        shared.fill(0);
    }
    Ok(keys)
}
