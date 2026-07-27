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

pub const MAGIC: u32 = 0x4e41_5544;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_STREAM_OPEN: u16 = 4;
pub const OP_FEED_PCM: u16 = 5;
pub const OP_PAUSE: u16 = 6;
pub const OP_CLOSE: u16 = 7;
pub const OP_RESUME: u16 = 8;

pub const E_AGAIN: i32 = -11;

fn build_request(op: u16, request_id: u32, payload: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HDR_LEN + payload.len());
    buf.extend_from_slice(&MAGIC.to_le_bytes());
    buf.extend_from_slice(&VERSION.to_le_bytes());
    buf.extend_from_slice(&op.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&request_id.to_le_bytes());
    buf.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    buf.extend_from_slice(payload);
    buf
}

pub fn open_request(request_id: u32, format: u16) -> Vec<u8> {
    build_request(OP_STREAM_OPEN, request_id, &format.to_le_bytes())
}

pub fn feed_request(request_id: u32, stream_id: u32, pcm: &[i16]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(8 + pcm.len() * 2);
    payload.extend_from_slice(&stream_id.to_le_bytes());
    payload.extend_from_slice(&((pcm.len() / 2) as u32).to_le_bytes());
    for sample in pcm {
        payload.extend_from_slice(&sample.to_le_bytes());
    }
    build_request(OP_FEED_PCM, request_id, &payload)
}

pub fn pause_request(request_id: u32, stream_id: u32) -> Vec<u8> {
    build_request(OP_PAUSE, request_id, &stream_id.to_le_bytes())
}

pub fn resume_request(request_id: u32, stream_id: u32) -> Vec<u8> {
    build_request(OP_RESUME, request_id, &stream_id.to_le_bytes())
}

pub fn close_request(request_id: u32, stream_id: u32) -> Vec<u8> {
    build_request(OP_CLOSE, request_id, &stream_id.to_le_bytes())
}

pub fn read_status(resp: &[u8]) -> i32 {
    let mut b = [0u8; 4];
    b.copy_from_slice(&resp[20..24]);
    i32::from_le_bytes(b)
}

pub fn read_stream_id(resp: &[u8]) -> u32 {
    let mut b = [0u8; 4];
    b.copy_from_slice(&resp[24..28]);
    u32::from_le_bytes(b)
}
