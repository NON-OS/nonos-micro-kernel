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

/// Header shared by every net.nym request and reply.
pub const HDR_LEN: usize = 20;

/// Wire magic, matching the mixnet capsule's protocol.
pub const MAGIC: u32 = 0x4E59_4D31;

/// Frame a request. Layout is magic, version, op, errno, reserved, request id,
/// payload length, then the payload.
pub fn encode(op: u16, request_id: u32, payload: &[u8], out: &mut [u8]) -> Option<usize> {
    let total = HDR_LEN.checked_add(payload.len())?;
    if out.len() < total {
        return None;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&1u16.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].fill(0);
    out[10..12].fill(0);
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    out[HDR_LEN..total].copy_from_slice(payload);
    Some(total)
}

/// Errno and payload from a reply, or None if it is short or not ours.
pub fn decode(buf: &[u8]) -> Option<(u16, &[u8])> {
    if buf.len() < HDR_LEN {
        return None;
    }
    if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC {
        return None;
    }
    let errno = u16::from_le_bytes([buf[8], buf[9]]);
    let len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
    let end = HDR_LEN.checked_add(len)?;
    if buf.len() < end {
        return None;
    }
    Some((errno, &buf[HDR_LEN..end]))
}
