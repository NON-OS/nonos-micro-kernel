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

use crate::protocol::errno::{E_BAD_LEN, E_BAD_VERSION};

pub const HDR_LEN: usize = 20;
/// Largest TCP payload a client may hand over in one send, which is the MSS
/// an Ethernet link gives it.
pub const SEGMENT_PAYLOAD_MAX: usize = 1460;
/// Room for that segment plus the fields wrapped around it. A buffer shorter
/// than the largest request a client may legally make does not fail loudly:
/// the kernel copies what fits and reports that count, so the request arrives
/// truncated and is rejected as malformed rather than as too long.
pub const IPC_BUF_MAX: usize = SEGMENT_PAYLOAD_MAX + 64;

#[derive(Clone, Copy)]
pub struct Request {
    pub magic: u32,
    pub op: u16,
    pub request_id: u32,
}

pub fn parse(buf: &[u8]) -> Result<(Request, &[u8]), u16> {
    if buf.len() < HDR_LEN {
        return Err(E_BAD_LEN);
    }
    let magic = le32(buf, 0);
    if le16(buf, 4) != 1 {
        return Err(E_BAD_VERSION);
    }
    let op = le16(buf, 6);
    let request_id = le32(buf, 12);
    let payload_len = le32(buf, 16) as usize;
    let want = HDR_LEN.checked_add(payload_len).ok_or(E_BAD_LEN)?;
    if buf.len() < want {
        return Err(E_BAD_LEN);
    }
    Ok((Request { magic, op, request_id }, &buf[HDR_LEN..want]))
}

fn le16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn le32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
