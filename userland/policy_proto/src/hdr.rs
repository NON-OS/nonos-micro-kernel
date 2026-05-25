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

pub const HDR_LEN: usize = 12;

#[derive(Clone, Copy)]
pub struct Header {
    pub op: u16,
    pub field: u32,
    pub kind: u8,
    pub status: u16,
    pub payload_len: u16,
}

impl Header {
    pub fn encode(&self, buf: &mut [u8]) {
        buf[0..2].copy_from_slice(&self.op.to_le_bytes());
        buf[2..6].copy_from_slice(&self.field.to_le_bytes());
        buf[6] = self.kind;
        buf[7] = 0;
        buf[8..10].copy_from_slice(&self.status.to_le_bytes());
        buf[10..12].copy_from_slice(&self.payload_len.to_le_bytes());
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        if buf.len() < HDR_LEN {
            return None;
        }
        Some(Header {
            op: u16::from_le_bytes([buf[0], buf[1]]),
            field: u32::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]),
            kind: buf[6],
            status: u16::from_le_bytes([buf[8], buf[9]]),
            payload_len: u16::from_le_bytes([buf[10], buf[11]]),
        })
    }
}
