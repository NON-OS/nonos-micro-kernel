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
use super::header::{Request, HDR_LEN, MAGIC, VERSION};
pub fn decode_request(buf: &[u8]) -> Option<Request> {
    if buf.len() < HDR_LEN {
        return None;
    }
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    if magic != MAGIC {
        return None;
    }
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    if version != VERSION {
        return None;
    }
    let op = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let flags = u16::from_le_bytes(buf[8..10].try_into().ok()?);
    let request_id = u32::from_le_bytes(buf[12..16].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[16..20].try_into().ok()?);
    Some(Request { op, flags, request_id, payload_len })
}
fn read_array<const N: usize>(buf: &[u8], offset: usize) -> Option<[u8; N]> {
    let end = offset.checked_add(N)?;
    let src = buf.get(offset..end)?;
    let mut out = [0u8; N];
    out.copy_from_slice(src);
    Some(out)
}
pub fn read_u32_le(buf: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(read_array(buf, offset)?))
}
pub fn read_u64_le(buf: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(read_array(buf, offset)?))
}
