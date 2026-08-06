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

use super::types::{EPK_LEN, HOP_BYTES, PREFIX_LEN, ROUTE_HEADER_LEN};
use super::{blocks, mask};
use crate::crypto::{fill_random, x25519_public};
use crate::packet::PacketError;
use crate::topology::{Node, ROUTE_HOPS};

pub fn build(
    id: u32,
    flags: u8,
    seed: &[u8; 32],
    cred: &[u8; 32],
    hops: &[Node; ROUTE_HOPS],
) -> Result<[u8; ROUTE_HEADER_LEN], PacketError> {
    let mut private = [0u8; 32];
    fill_random(&mut private).map_err(|_| PacketError::Crypto)?;
    let mut out = [0u8; ROUTE_HEADER_LEN];
    let mut public = [0u8; EPK_LEN];
    x25519_public(&private, &mut public).map_err(|_| PacketError::Crypto)?;
    out[..EPK_LEN].copy_from_slice(&public);
    out[32] = 1;
    out[33] = ROUTE_HOPS as u8;
    let mut keys = blocks::write(&mut out, id, flags, seed, cred, hops, &private)?;
    let mut mask_err = None;
    for idx in (0..ROUTE_HOPS).rev() {
        let start = PREFIX_LEN + idx * HOP_BYTES;
        if let Err(e) = mask::apply(&mut out[start..], seed, idx as u8, &keys[idx]) {
            mask_err = Some(e);
            break;
        }
    }
    private.fill(0);
    public.fill(0);
    for key in keys.iter_mut() {
        key.fill(0);
    }
    if let Some(e) = mask_err {
        return Err(e);
    }
    Ok(out)
}
