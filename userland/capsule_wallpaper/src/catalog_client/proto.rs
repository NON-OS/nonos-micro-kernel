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

pub const SERVICE_NAME: &[u8] = b"wallpaper_catalog";

pub const OP_GET_SIZE: u16 = 0x0002;
pub const OP_GET_CHUNK: u16 = 0x0003;

pub const E_OK: u16 = 0;
pub const HDR_LEN: usize = 16;
pub const CHUNK_MAX: usize = 4096;
pub const IPC_PAYLOAD_MAX: usize = CHUNK_MAX + 32;

#[derive(Clone, Copy)]
pub struct Header {
    pub op: u16,
    pub status: u16,
    pub index: u32,
    pub offset: u32,
    pub payload_len: u32,
}

impl Header {
    pub fn encode(&self, buf: &mut [u8]) {
        buf[0..2].copy_from_slice(&self.op.to_le_bytes());
        buf[2..4].copy_from_slice(&self.status.to_le_bytes());
        buf[4..8].copy_from_slice(&self.index.to_le_bytes());
        buf[8..12].copy_from_slice(&self.offset.to_le_bytes());
        buf[12..16].copy_from_slice(&self.payload_len.to_le_bytes());
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        if buf.len() < HDR_LEN {
            return None;
        }
        Some(Header {
            op: u16::from_le_bytes([buf[0], buf[1]]),
            status: u16::from_le_bytes([buf[2], buf[3]]),
            index: u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
            offset: u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
            payload_len: u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]),
        })
    }
}
