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

//! The net_core link protocol as the driver side sees it. net_core drives every
//! network interface, wired or wireless, with the same 20-byte envelope and
//! four operations; a WiFi driver becomes a bound interface by answering them.
//! These constants and helpers mirror capsule_net_core/src/protocol, kept local
//! so the adapter stays self-contained.

/// "NNET" little-endian, the tag on every net_core link message.
pub const MAGIC_NNET: u32 = 0x4E4E_4554;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;

pub const OP_LINK_STATUS: u16 = 2;
pub const OP_MAC_ADDRESS: u16 = 3;
pub const OP_TX_PACKET: u16 = 4;
pub const OP_RX_PACKET: u16 = 5;

/// The largest 802.3 frame carried in either direction (1500 MTU + header).
pub const MAX_FRAME: usize = 1514;

/// Status meaning "no frame right now"; net_core treats any nonzero receive
/// status as an empty poll and retries.
pub const STATUS_AGAIN: i32 = 8;
/// Status meaning "the link is down" (not associated).
pub const STATUS_NO_LINK: i32 = 5;

/// A parsed request: its operation, the request id to echo, and the payload
/// bytes after the header (a frame for a transmit, empty otherwise).
pub struct Request<'a> {
    pub op: u16,
    pub request_id: u32,
    pub payload: &'a [u8],
}

/// Parse a net_core request header, rejecting anything that is not a well-formed
/// message for this version.
pub fn parse_request(buf: &[u8]) -> Option<Request<'_>> {
    if buf.len() < HDR_LEN {
        return None;
    }
    if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC_NNET {
        return None;
    }
    if u16::from_le_bytes([buf[4], buf[5]]) != VERSION {
        return None;
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
    let end = HDR_LEN.checked_add(payload_len)?;
    if end > buf.len() {
        return None;
    }
    Some(Request { op, request_id, payload: &buf[HDR_LEN..end] })
}

/// Write a response header into `out`, returning the header length or None if
/// the buffer is too small.
pub fn write_response_header(
    out: &mut [u8],
    op: u16,
    request_id: u32,
    payload_len: u32,
) -> Option<usize> {
    if out.len() < HDR_LEN {
        return None;
    }
    out[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
    Some(HDR_LEN)
}
