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
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_recv, mk_ipc_send};

pub const NCMP_MAGIC: u32 = 0x4E43_4D50;
pub const NCMP_VERSION: u16 = 1;
pub const NCMP_HDR_LEN: usize = 20;
const REPLY_TIMEOUT_MS: u64 = 16;

pub fn build_request(out: &mut Vec<u8>, op: u16, request_id: u32, payload: &[u8]) {
    out.clear();
    out.extend_from_slice(&NCMP_MAGIC.to_le_bytes());
    out.extend_from_slice(&NCMP_VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}

pub fn call(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(NCMP_HDR_LEN + payload.len());
    build_request(&mut tx, op, request_id, payload);
    let mut rx = vec![0u8; NCMP_HDR_LEN + 4];
    let send_rc = mk_ipc_send(compositor_port as u64, tx.as_ptr(), tx.len());
    if send_rc < 0 {
        return Err("compositor send failed");
    }
    let rc = mk_ipc_recv(0, rx.as_mut_ptr(), rx.len(), REPLY_TIMEOUT_MS);
    if rc < (NCMP_HDR_LEN + 4) as i64 {
        return Err("compositor call failed");
    }
    Ok(i32::from_le_bytes(
        rx[NCMP_HDR_LEN..NCMP_HDR_LEN + 4].try_into().map_err(|_| "compositor short response")?,
    ))
}

pub fn call_payload(
    compositor_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    body: &mut [u8],
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(NCMP_HDR_LEN + payload.len());
    build_request(&mut tx, op, request_id, payload);
    let mut rx = vec![0u8; NCMP_HDR_LEN + 4 + body.len()];
    let send_rc = mk_ipc_send(compositor_port as u64, tx.as_ptr(), tx.len());
    if send_rc < 0 {
        return Err("compositor send failed");
    }
    let rc = mk_ipc_recv(0, rx.as_mut_ptr(), rx.len(), REPLY_TIMEOUT_MS);
    if rc < (NCMP_HDR_LEN + 4 + body.len()) as i64 {
        return Err("compositor call failed");
    }
    body.copy_from_slice(&rx[NCMP_HDR_LEN + 4..NCMP_HDR_LEN + 4 + body.len()]);
    Ok(i32::from_le_bytes(
        rx[NCMP_HDR_LEN..NCMP_HDR_LEN + 4].try_into().map_err(|_| "compositor short response")?,
    ))
}
