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

//! IEEE 802.11 MAC frame header, pure encoding with no hardware. Every
//! management frame the association MLME sends (probe request, authentication,
//! association request) and every EAPOL frame the WPA2 handshake exchanges is
//! built on this header. The field layout is the IEEE 802.11 standard and is
//! stable across chip families, so this is correct by construction and is
//! checked by the `iwlwifi_proofs` crate rather than needing hardware.

pub type MacAddr = [u8; 6];

/// The all-ones address: a wildcard receiver/BSSID for a scanning probe.
pub const BROADCAST: MacAddr = [0xFF; 6];

// Frame Control type field (bits 2-3 of octet 0).
pub const TYPE_MGMT: u8 = 0;

// Management-frame subtypes (bits 4-7 of octet 0) the driver builds or parses.
pub const SUBTYPE_ASSOC_REQ: u8 = 0;
pub const SUBTYPE_PROBE_REQ: u8 = 4;
pub const SUBTYPE_PROBE_RESP: u8 = 5;
pub const SUBTYPE_BEACON: u8 = 8;
pub const SUBTYPE_AUTH: u8 = 11;

/// A management or data MAC header without QoS or a fourth address is 24 bytes:
/// frame control (2), duration (2), addr1/2/3 (6 each), sequence control (2).
pub const MAC_HEADER_LEN: usize = 24;

/// Build the 2-byte Frame Control value: protocol version 0, the given type and
/// subtype, no flags. Stored little-endian on the wire.
pub const fn frame_control(ftype: u8, subtype: u8) -> u16 {
    ((ftype as u16 & 0x3) << 2) | ((subtype as u16 & 0xF) << 4)
}

/// The type carried by a frame-control value.
pub const fn fc_type(fc: u16) -> u8 {
    ((fc >> 2) & 0x3) as u8
}

/// The subtype carried by a frame-control value.
pub const fn fc_subtype(fc: u16) -> u8 {
    ((fc >> 4) & 0xF) as u8
}

/// The sequence-control value: a 12-bit sequence number in the high bits and a
/// 4-bit fragment number of zero. Stored little-endian.
pub const fn seq_control(seq: u16) -> u16 {
    (seq & 0x0FFF) << 4
}

/// Write a management or data MAC header into `out`, returning the byte count
/// (`MAC_HEADER_LEN`) or `None` if the buffer is too small. For the frames the
/// MLME sends: addr1 is the receiver, addr2 the transmitter, addr3 the BSSID.
pub fn write_header(
    out: &mut [u8],
    fc: u16,
    addr1: MacAddr,
    addr2: MacAddr,
    addr3: MacAddr,
    seq: u16,
) -> Option<usize> {
    if out.len() < MAC_HEADER_LEN {
        return None;
    }
    out[0..2].copy_from_slice(&fc.to_le_bytes());
    out[2..4].copy_from_slice(&0u16.to_le_bytes());
    out[4..10].copy_from_slice(&addr1);
    out[10..16].copy_from_slice(&addr2);
    out[16..22].copy_from_slice(&addr3);
    out[22..24].copy_from_slice(&seq_control(seq).to_le_bytes());
    Some(MAC_HEADER_LEN)
}
