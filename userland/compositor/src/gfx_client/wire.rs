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

use nonos_libc::mk_ipc_call_timeout;

pub const NVGP_MAGIC: u32 = 0x4E56_4750;
pub const NVGP_VERSION: u16 = 1;
pub const NVGP_HDR_LEN: usize = 20;
const CALL_REPLY_TIMEOUT_MS: u64 = 1000;
const BOOT_REPLY_TIMEOUT_MS: u64 = 250;

pub fn build_request(out: &mut Vec<u8>, op: u16, request_id: u32, payload: &[u8]) {
    out.clear();
    out.extend_from_slice(&NVGP_MAGIC.to_le_bytes());
    out.extend_from_slice(&NVGP_VERSION.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}

pub fn call(
    gfx_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    rx: &mut Vec<u8>,
) -> Result<usize, &'static str> {
    call_with_timeout(gfx_port, op, request_id, payload, rx, CALL_REPLY_TIMEOUT_MS)
}

pub fn call_boot(
    gfx_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    rx: &mut Vec<u8>,
) -> Result<usize, &'static str> {
    call_with_timeout(gfx_port, op, request_id, payload, rx, BOOT_REPLY_TIMEOUT_MS)
}

fn call_with_timeout(
    gfx_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    rx: &mut Vec<u8>,
    timeout_ms: u64,
) -> Result<usize, &'static str> {
    let mut tx = Vec::with_capacity(NVGP_HDR_LEN + payload.len());
    build_request(&mut tx, op, request_id, payload);
    let rc = mk_ipc_call_timeout(
        gfx_port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    if rc <= 0 {
        return Err("gfx ipc call failed");
    }
    Ok(rc as usize)
}

pub fn read_status(buf: &[u8]) -> Option<i32> {
    if buf.len() < NVGP_HDR_LEN + 4 {
        return None;
    }
    Some(i32::from_le_bytes(buf[NVGP_HDR_LEN..NVGP_HDR_LEN + 4].try_into().ok()?))
}

pub fn payload_slice(buf: &[u8]) -> &[u8] {
    let off = NVGP_HDR_LEN + 4;
    if buf.len() < off {
        return &[];
    }
    &buf[off..]
}
