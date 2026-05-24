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

use super::constants::*;
use crate::clients::envelope::call;

pub fn create_surb(port: u32, session: u32) -> Result<(u32, [u8; 32]), u16> {
    let body = session.to_le_bytes();
    create_with_body(port, &body)
}

pub fn create_surb_with_ttl(
    port: u32,
    session: u32,
    ttl_ms: u64,
) -> Result<(u32, [u8; 32]), u16> {
    let mut body = [0u8; 12];
    body[0..4].copy_from_slice(&session.to_le_bytes());
    body[4..12].copy_from_slice(&ttl_ms.to_le_bytes());
    create_with_body(port, &body)
}

fn create_with_body(port: u32, body: &[u8]) -> Result<(u32, [u8; 32]), u16> {
    let mut out = [0u8; 36];
    if call(port, MAGIC, CREATE_SURB, body, &mut out)? != 36 {
        return Err(4);
    }
    let mut tag = [0u8; 32];
    tag.copy_from_slice(&out[4..36]);
    Ok((u32::from_le_bytes([out[0], out[1], out[2], out[3]]), tag))
}

pub fn send_reply(port: u32, surb: u32, tag: &[u8; 32], payload: &[u8]) -> Result<(), u16> {
    let mut body = vec![0u8; 36 + payload.len()];
    body[0..4].copy_from_slice(&surb.to_le_bytes());
    body[4..36].copy_from_slice(tag);
    body[36..].copy_from_slice(payload);
    call(port, MAGIC, SEND_REPLY, &body, &mut []).map(|_| ())
}
