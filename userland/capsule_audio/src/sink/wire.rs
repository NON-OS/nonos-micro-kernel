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

pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
pub const MAX_PCM_CHUNK: usize = 4096;

const MAGIC: u32 = 0x4e48_4441;
const VERSION: u16 = 1;
const OP_WRITE_PCM: u16 = 7;
const OP_STREAM_START: u16 = 8;
const E_OK: i32 = 0;

pub fn request(request_id: u32, pcm: &[u8], out: &mut [u8]) -> usize {
    let payload_len = pcm.len();
    if payload_len == 0 || payload_len > MAX_PCM_CHUNK || out.len() < HDR_LEN + payload_len {
        return 0;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&OP_WRITE_PCM.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&(payload_len as u32).to_le_bytes());
    out[HDR_LEN..HDR_LEN + payload_len].copy_from_slice(pcm);
    HDR_LEN + payload_len
}

pub fn start_request(request_id: u32, out: &mut [u8]) -> usize {
    if out.len() < HDR_LEN {
        return 0;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&OP_STREAM_START.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&0u32.to_le_bytes());
    HDR_LEN
}

pub fn reply_ok(rx: &[u8]) -> bool {
    if rx.len() < HDR_LEN + STATUS_LEN {
        return false;
    }
    let mut status = [0u8; STATUS_LEN];
    status.copy_from_slice(&rx[HDR_LEN..HDR_LEN + STATUS_LEN]);
    i32::from_le_bytes(status) == E_OK
}

pub fn reply_status(rx: &[u8]) -> i32 {
    if rx.len() < HDR_LEN + STATUS_LEN {
        return i32::MIN;
    }
    let mut status = [0u8; STATUS_LEN];
    status.copy_from_slice(&rx[HDR_LEN..HDR_LEN + STATUS_LEN]);
    i32::from_le_bytes(status)
}
