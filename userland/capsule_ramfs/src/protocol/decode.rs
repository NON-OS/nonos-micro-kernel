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

use super::types::{Request, HDR_LEN};

pub fn decode_request(buf: &[u8]) -> Option<Request<'_>> {
    if buf.len() < HDR_LEN {
        return None;
    }
    let reserved = u16::from_le_bytes([buf[6], buf[7]]);
    if reserved != 0 {
        return None;
    }
    let seq = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let op = u16::from_le_bytes([buf[4], buf[5]]);
    Some(Request { seq, op, payload: &buf[HDR_LEN..] })
}

fn read_array<const N: usize>(buf: &[u8], offset: usize) -> Option<[u8; N]> {
    let end = offset.checked_add(N)?;
    let src = buf.get(offset..end)?;
    let mut out = [0u8; N];
    out.copy_from_slice(src);
    Some(out)
}

pub fn read_u16_le(buf: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_le_bytes(read_array(buf, offset)?))
}

pub fn read_u32_le(buf: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(read_array(buf, offset)?))
}

pub fn read_u64_le(buf: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(read_array(buf, offset)?))
}
