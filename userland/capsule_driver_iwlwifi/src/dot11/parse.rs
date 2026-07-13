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

//! Parsing of 802.11 beacon and probe-response frames, the counterpart to the
//! management-frame builders. A scan turns received beacons into a list of
//! visible networks: BSSID, SSID, operating channel, and whether the network
//! advertises RSN (WPA2). Pure parsing over untrusted input with a bounded
//! walk and no panics, checked by `iwlwifi_proofs`.

use super::header::{fc_subtype, fc_type, MAC_HEADER_LEN, SUBTYPE_BEACON, SUBTYPE_PROBE_RESP, TYPE_MGMT};

/// Beacon and probe-response fixed fields after the MAC header: timestamp (8),
/// beacon interval (2), capability info (2).
pub const FIXED_FIELDS_LEN: usize = 12;

/// Information-element ids a scan reads.
pub const IE_SSID: u8 = 0;
pub const IE_DS_PARAMS: u8 = 3;
pub const IE_RSN: u8 = 48;

/// A parsed view of a beacon or probe response. `ssid` borrows the frame.
pub struct BeaconInfo<'a> {
    pub bssid: [u8; 6],
    pub capability: u16,
    pub ssid: &'a [u8],
    pub channel: u8,
    pub rsn: bool,
}

/// Find an information element by id in a tagged-parameter area. Walks
/// `[id][len][data]` tuples with a bounded index and returns the element's data
/// slice, or `None` if it is absent or a length runs past the buffer.
pub fn find_ie(tags: &[u8], id: u8) -> Option<&[u8]> {
    let mut off = 0usize;
    while off + 2 <= tags.len() {
        let eid = tags[off];
        let len = tags[off + 1] as usize;
        let start = off + 2;
        let end = start.checked_add(len)?;
        if end > tags.len() {
            return None;
        }
        if eid == id {
            return Some(&tags[start..end]);
        }
        off = end;
    }
    None
}

/// Parse a beacon or probe-response frame that starts at the MAC header.
/// Returns `None` if the frame is too short for the header and fixed fields, or
/// if it carries no SSID element. A missing DS-parameter element leaves the
/// channel at zero; an RSN element sets `rsn`.
pub fn parse_beacon(frame: &[u8]) -> Option<BeaconInfo<'_>> {
    if frame.len() < MAC_HEADER_LEN + FIXED_FIELDS_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT || (fc_subtype(fc) != SUBTYPE_BEACON && fc_subtype(fc) != SUBTYPE_PROBE_RESP) {
        return None;
    }
    let mut bssid = [0u8; 6];
    bssid.copy_from_slice(&frame[16..22]);
    let fixed = MAC_HEADER_LEN;
    let capability = u16::from_le_bytes([frame[fixed + 10], frame[fixed + 11]]);
    let tags = &frame[MAC_HEADER_LEN + FIXED_FIELDS_LEN..];
    let ssid = find_ie(tags, IE_SSID)?;
    let channel = match find_ie(tags, IE_DS_PARAMS) {
        Some(ds) if !ds.is_empty() => ds[0],
        _ => 0,
    };
    let rsn = find_ie(tags, IE_RSN).is_some();
    Some(BeaconInfo { bssid, capability, ssid, channel, rsn })
}
