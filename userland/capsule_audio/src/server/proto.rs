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

pub const MAGIC: u32 = 0x4e41_5544;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
pub const OP_PLAY_TONE: u16 = 1;
pub const OP_PLAY_PCM: u16 = 2;
pub const OP_STOP: u16 = 3;
pub const E_OK: i32 = 0;
pub const E_INVAL: i32 = -22;

pub struct Request {
    pub op: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

pub fn decode(msg: &[u8]) -> Option<Request> {
    if msg.len() < HDR_LEN {
        return None;
    }
    let magic = u32::from_le_bytes([msg[0], msg[1], msg[2], msg[3]]);
    let version = u16::from_le_bytes([msg[4], msg[5]]);
    if magic != MAGIC || version != VERSION {
        return None;
    }
    Some(Request {
        op: u16::from_le_bytes([msg[6], msg[7]]),
        request_id: u32::from_le_bytes([msg[12], msg[13], msg[14], msg[15]]),
        payload_len: u32::from_le_bytes([msg[16], msg[17], msg[18], msg[19]]),
    })
}

pub fn write_header(out: &mut [u8], op: u16, request_id: u32, payload_len: u32) {
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..12].copy_from_slice(&0u32.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
}

pub fn encode_reply(req: &Request, status: i32, out: &mut [u8]) -> usize {
    if out.len() < HDR_LEN + STATUS_LEN {
        return 0;
    }
    write_header(out, req.op, req.request_id, STATUS_LEN as u32);
    out[HDR_LEN..HDR_LEN + STATUS_LEN].copy_from_slice(&status.to_le_bytes());
    HDR_LEN + STATUS_LEN
}
