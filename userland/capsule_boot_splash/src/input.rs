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

use nonos_libc::mk_ipc_call;

const REQ_MAGIC: u32 = 0x4E49_5253;
const REQ_VERSION: u16 = 1;
const REQ_HDR_LEN: usize = 20;
const OP_SUBSCRIBE: u16 = 0x0002;
const OP_GRAB_REQUEST: u16 = 0x0003;
const OP_GRAB_RELEASE: u16 = 0x0004;
const KIND_MASK_KEYS: u32 = 0b11;
const DELIVERY_MAGIC: u32 = 0x4E49_4E50;
pub(crate) const DELIVERY_LEN: usize = 40;

fn request(port: u32, op: u16, rid: u32, body: &[u8]) -> Result<(), i32> {
    let mut tx = [0u8; REQ_HDR_LEN + 8];
    tx[0..4].copy_from_slice(&REQ_MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&REQ_VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[12..16].copy_from_slice(&rid.to_le_bytes());
    tx[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    tx[REQ_HDR_LEN..REQ_HDR_LEN + body.len()].copy_from_slice(body);
    let mut rx = [0u8; REQ_HDR_LEN + 4];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), REQ_HDR_LEN + body.len(), rx.as_mut_ptr(), rx.len());
    if rc < (REQ_HDR_LEN + 4) as i64 {
        return Err(-11);
    }
    let s = i32::from_le_bytes(rx[REQ_HDR_LEN..REQ_HDR_LEN + 4].try_into().map_err(|_| -11)?);
    if s == 0 { Ok(()) } else { Err(s) }
}

fn keys_body() -> [u8; 8] {
    let mut b = [0u8; 8];
    b[0..4].copy_from_slice(&KIND_MASK_KEYS.to_le_bytes());
    b
}

pub(crate) fn subscribe(port: u32, rid: u32) -> Result<(), i32> {
    request(port, OP_SUBSCRIBE, rid, &keys_body())
}

pub(crate) fn grab(port: u32, rid: u32) -> Result<(), i32> {
    request(port, OP_GRAB_REQUEST, rid, &keys_body())
}

pub(crate) fn release(port: u32, rid: u32) -> Result<(), i32> {
    request(port, OP_GRAB_RELEASE, rid, &[])
}

pub(crate) fn parse_key(buf: &[u8]) -> Option<(u16, u32)> {
    if buf.len() < DELIVERY_LEN {
        return None;
    }
    if u32::from_le_bytes(buf[0..4].try_into().ok()?) != DELIVERY_MAGIC {
        return None;
    }
    let kind = u16::from_le_bytes(buf[8..10].try_into().ok()?);
    let code = u32::from_le_bytes(buf[12..16].try_into().ok()?);
    Some((kind, code))
}
