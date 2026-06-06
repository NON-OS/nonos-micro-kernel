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

pub const HDR_LEN: usize = 14;
pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const BROADCAST: [u8; 6] = [0xFF; 6];

pub fn write(out: &mut [u8], dst_mac: &[u8; 6], src_mac: &[u8; 6], ethertype: u16) -> usize {
    if out.len() < HDR_LEN {
        return 0;
    }
    out[0..6].copy_from_slice(dst_mac);
    out[6..12].copy_from_slice(src_mac);
    out[12..14].copy_from_slice(&ethertype.to_be_bytes());
    HDR_LEN
}

pub fn parse(bytes: &[u8]) -> Option<([u8; 6], [u8; 6], u16)> {
    if bytes.len() < HDR_LEN {
        return None;
    }
    let mut dst = [0u8; 6];
    let mut src = [0u8; 6];
    dst.copy_from_slice(&bytes[0..6]);
    src.copy_from_slice(&bytes[6..12]);
    let et = u16::from_be_bytes([bytes[12], bytes[13]]);
    Some((dst, src, et))
}
