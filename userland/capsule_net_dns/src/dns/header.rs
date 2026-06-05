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

pub const FLAG_QR: u16 = 0x8000;
pub const FLAG_RD: u16 = 0x0100;
pub const RCODE_MASK: u16 = 0x000F;

pub const RCODE_NO_ERROR: u16 = 0;
pub const RCODE_NXDOMAIN: u16 = 3;

#[derive(Clone, Copy, Default)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
}

impl Header {
    pub fn parse(message: &[u8]) -> Option<Self> {
        if message.len() < HDR_LEN {
            return None;
        }
        Some(Self {
            id: u16::from_be_bytes([message[0], message[1]]),
            flags: u16::from_be_bytes([message[2], message[3]]),
            qdcount: u16::from_be_bytes([message[4], message[5]]),
            ancount: u16::from_be_bytes([message[6], message[7]]),
        })
    }

    pub fn is_response(&self) -> bool {
        self.flags & FLAG_QR != 0
    }

    pub fn rcode(&self) -> u16 {
        self.flags & RCODE_MASK
    }
}
