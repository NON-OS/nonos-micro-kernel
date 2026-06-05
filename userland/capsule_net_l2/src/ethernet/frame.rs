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

use super::types::MacAddress;

pub const HDR_LEN: usize = 14;

#[derive(Clone, Copy)]
pub struct EthHeader {
    pub dst: MacAddress,
    pub src: MacAddress,
    pub ethertype: u16,
}

impl EthHeader {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < HDR_LEN {
            return None;
        }
        let mut dst = [0u8; 6];
        let mut src = [0u8; 6];
        dst.copy_from_slice(&bytes[0..6]);
        src.copy_from_slice(&bytes[6..12]);
        let ethertype = u16::from_be_bytes([bytes[12], bytes[13]]);
        Some(Self { dst, src, ethertype })
    }

    pub fn write(&self, out: &mut [u8]) -> bool {
        if out.len() < HDR_LEN {
            return false;
        }
        out[0..6].copy_from_slice(&self.dst);
        out[6..12].copy_from_slice(&self.src);
        out[12..14].copy_from_slice(&self.ethertype.to_be_bytes());
        true
    }
}

#[inline]
pub fn payload_of(bytes: &[u8]) -> Option<&[u8]> {
    if bytes.len() < HDR_LEN {
        None
    } else {
        Some(&bytes[HDR_LEN..])
    }
}
