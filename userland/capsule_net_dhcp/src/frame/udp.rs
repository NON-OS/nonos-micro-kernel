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

use super::checksum::fold_with_pseudo;
use super::ipv4::PROTO_UDP;

pub const HDR_LEN: usize = 8;

pub fn write(
    out: &mut [u8],
    src: &[u8; 4],
    dst: &[u8; 4],
    src_port: u16,
    dst_port: u16,
    payload: &[u8],
) -> usize {
    let total = HDR_LEN + payload.len();
    if out.len() < total {
        return 0;
    }
    out[0..2].copy_from_slice(&src_port.to_be_bytes());
    out[2..4].copy_from_slice(&dst_port.to_be_bytes());
    out[4..6].copy_from_slice(&(total as u16).to_be_bytes());
    out[6..8].copy_from_slice(&[0, 0]);
    out[HDR_LEN..total].copy_from_slice(payload);
    let cs = fold_with_pseudo(src, dst, PROTO_UDP, &out[..total]);
    out[6..8].copy_from_slice(&cs.to_be_bytes());
    total
}

pub fn parse(bytes: &[u8]) -> Option<(u16, u16, usize)> {
    if bytes.len() < HDR_LEN {
        return None;
    }
    let src_port = u16::from_be_bytes([bytes[0], bytes[1]]);
    let dst_port = u16::from_be_bytes([bytes[2], bytes[3]]);
    let length = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    if length < HDR_LEN || length > bytes.len() {
        return None;
    }
    Some((src_port, dst_port, length))
}
