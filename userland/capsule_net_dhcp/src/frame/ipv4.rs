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

use super::checksum::fold;

pub const HDR_LEN: usize = 20;
pub const VERSION_4: u8 = 4;
pub const DEFAULT_TTL: u8 = 64;
pub const PROTO_UDP: u8 = 17;

pub fn write(
    out: &mut [u8],
    src: &[u8; 4],
    dst: &[u8; 4],
    proto: u8,
    identification: u16,
    total_len: u16,
) -> usize {
    if out.len() < HDR_LEN {
        return 0;
    }
    out[0] = (VERSION_4 << 4) | 5;
    out[1] = 0;
    out[2..4].copy_from_slice(&total_len.to_be_bytes());
    out[4..6].copy_from_slice(&identification.to_be_bytes());
    out[6] = 0x40;
    out[7] = 0;
    out[8] = DEFAULT_TTL;
    out[9] = proto;
    out[10] = 0;
    out[11] = 0;
    out[12..16].copy_from_slice(src);
    out[16..20].copy_from_slice(dst);
    let cs = fold(&out[..HDR_LEN]);
    out[10..12].copy_from_slice(&cs.to_be_bytes());
    HDR_LEN
}

pub fn parse(bytes: &[u8]) -> Option<([u8; 4], [u8; 4], u8, usize)> {
    if bytes.len() < HDR_LEN {
        return None;
    }
    if (bytes[0] >> 4) != VERSION_4 {
        return None;
    }
    let ihl_words = (bytes[0] & 0x0F) as usize;
    if ihl_words < 5 {
        return None;
    }
    let header_len = ihl_words * 4;
    if bytes.len() < header_len {
        return None;
    }
    let proto = bytes[9];
    let mut src = [0u8; 4];
    let mut dst = [0u8; 4];
    src.copy_from_slice(&bytes[12..16]);
    dst.copy_from_slice(&bytes[16..20]);
    Some((src, dst, proto, header_len))
}
