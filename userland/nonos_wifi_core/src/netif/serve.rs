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

//! Answer net_core's link operations for a WiFi interface. The four operations
//! are the whole contract: report the link state and the station MAC, take
//! Ethernet frames to transmit, and hand back received Ethernet frames. All the
//! WiFi specifics (association, CCMP, the firmware rings) live behind the
//! `LinkPort` so this translation is exact and testable without a radio.

use super::wire::{
    parse_request, write_response_header, HDR_LEN, MAX_FRAME, OP_LINK_STATUS, OP_MAC_ADDRESS,
    OP_RX_PACKET, OP_TX_PACKET, STATUS_AGAIN, STATUS_NO_LINK,
};

/// The WiFi link as net_core needs to see it: an Ethernet-framed interface.
/// Implementations translate to and from 802.11 behind these methods.
pub trait LinkPort {
    /// The station MAC address once associated, else None.
    fn mac(&self) -> Option<[u8; 6]>;
    /// Whether the station is associated and carrying data.
    fn link_up(&self) -> bool;
    /// Copy the next received Ethernet frame into `out`, returning its length,
    /// or None when nothing is queued.
    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize>;
    /// Transmit one Ethernet frame; returns false if it could not be sent
    /// (for instance while unassociated).
    fn send_tx(&mut self, frame: &[u8]) -> bool;
}

/// Handle one net_core request, writing the response into `out` and returning
/// its length. Returns None only for a malformed request or an undersized
/// buffer; every known operation produces a well-formed response, including the
/// "no frame" and "link down" cases net_core expects.
pub fn serve<L: LinkPort>(req: &[u8], link: &mut L, out: &mut [u8]) -> Option<usize> {
    let r = parse_request(req)?;
    match r.op {
        OP_LINK_STATUS => status_byte(out, OP_LINK_STATUS, r.request_id, 0, link.link_up() as u8),
        OP_MAC_ADDRESS => mac_response(out, r.request_id, link.mac()),
        OP_TX_PACKET => {
            let status = if link.send_tx(r.payload) { 0 } else { -STATUS_NO_LINK };
            status_only(out, OP_TX_PACKET, r.request_id, status)
        }
        OP_RX_PACKET => rx_response(out, r.request_id, link),
        _ => None,
    }
}

// Response: header + i32 status.
fn status_only(out: &mut [u8], op: u16, rid: u32, status: i32) -> Option<usize> {
    let n = write_response_header(out, op, rid, 4)?;
    put_i32(out, n, status)?;
    Some(n + 4)
}

// Response: header + i32 status + one trailing byte (link up flag).
fn status_byte(out: &mut [u8], op: u16, rid: u32, status: i32, byte: u8) -> Option<usize> {
    let n = write_response_header(out, op, rid, 5)?;
    put_i32(out, n, status)?;
    *out.get_mut(n + 4)? = byte;
    Some(n + 5)
}

// Response: header + i32 status + 6-byte MAC. Unassociated yields a no-link
// status with a zeroed address, which net_core reads as "no MAC yet".
fn mac_response(out: &mut [u8], rid: u32, mac: Option<[u8; 6]>) -> Option<usize> {
    let n = write_response_header(out, OP_MAC_ADDRESS, rid, 4 + 6)?;
    let (status, addr) = match mac {
        Some(m) => (0, m),
        None => (-STATUS_NO_LINK, [0u8; 6]),
    };
    put_i32(out, n, status)?;
    out.get_mut(n + 4..n + 10)?.copy_from_slice(&addr);
    Some(n + 10)
}

// Response: header + i32 status + u32 frame_len + frame. An empty poll is a
// retry status with no frame, exactly what net_core drains against.
fn rx_response<L: LinkPort>(out: &mut [u8], rid: u32, link: &mut L) -> Option<usize> {
    let mut frame = [0u8; MAX_FRAME];
    match link.poll_rx(&mut frame) {
        Some(len) if len <= MAX_FRAME => {
            let n = write_response_header(out, OP_RX_PACKET, rid, (4 + 4 + len) as u32)?;
            put_i32(out, n, 0)?;
            out.get_mut(n + 4..n + 8)?.copy_from_slice(&(len as u32).to_le_bytes());
            out.get_mut(n + 8..n + 8 + len)?.copy_from_slice(&frame[..len]);
            Some(n + 8 + len)
        }
        _ => {
            let n = write_response_header(out, OP_RX_PACKET, rid, 4)?;
            put_i32(out, n, STATUS_AGAIN)?;
            Some(n + 4)
        }
    }
}

fn put_i32(out: &mut [u8], at: usize, val: i32) -> Option<()> {
    out.get_mut(at..at + 4)?.copy_from_slice(&val.to_le_bytes());
    Some(())
}

/// The largest response this adapter ever writes: an RX response carrying a
/// full frame. Callers size their output buffer with this.
pub const MAX_RESPONSE: usize = HDR_LEN + 4 + 4 + MAX_FRAME;
