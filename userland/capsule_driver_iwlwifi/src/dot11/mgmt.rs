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

//! 802.11 management frame builders for the association MLME: probe request
//! (scan), open-system authentication, and association request. Each writes a
//! complete frame into the caller's buffer and returns its length, or `None` on
//! overflow. Pure encoding; the driver hands the bytes to the firmware TX path.

use super::header::{
    frame_control, write_header, MacAddr, BROADCAST, SUBTYPE_ASSOC_REQ, SUBTYPE_AUTH,
    SUBTYPE_PROBE_REQ, TYPE_MGMT,
};

// Information element ids the builders emit.
pub const IE_SSID: u8 = 0;
pub const IE_SUPPORTED_RATES: u8 = 1;

/// Append an information element `[id][len][data]` at `off`, returning the new
/// offset or `None` if it would overflow the buffer or the 255-byte IE limit.
fn put_ie(out: &mut [u8], off: usize, id: u8, data: &[u8]) -> Option<usize> {
    if data.len() > 255 {
        return None;
    }
    let end = off.checked_add(2)?.checked_add(data.len())?;
    if end > out.len() {
        return None;
    }
    out[off] = id;
    out[off + 1] = data.len() as u8;
    out[off + 2..end].copy_from_slice(data);
    Some(end)
}

/// Build a probe request: a broadcast-addressed management frame carrying the
/// SSID element (empty for a wildcard scan, or the target for a directed probe)
/// and the supported-rates element. `src` is the station MAC.
pub fn probe_request(
    out: &mut [u8],
    src: MacAddr,
    ssid: &[u8],
    rates: &[u8],
    seq: u16,
) -> Option<usize> {
    let fc = frame_control(TYPE_MGMT, SUBTYPE_PROBE_REQ);
    let mut off = write_header(out, fc, BROADCAST, src, BROADCAST, seq)?;
    off = put_ie(out, off, IE_SSID, ssid)?;
    off = put_ie(out, off, IE_SUPPORTED_RATES, rates)?;
    Some(off)
}

/// Build an open-system authentication request: algorithm 0 (open), transaction
/// sequence 1, status 0. `bssid` is the target AP.
pub fn auth_open(out: &mut [u8], src: MacAddr, bssid: MacAddr, seq: u16) -> Option<usize> {
    let fc = frame_control(TYPE_MGMT, SUBTYPE_AUTH);
    let off = write_header(out, fc, bssid, src, bssid, seq)?;
    let end = off + 6;
    if end > out.len() {
        return None;
    }
    out[off..off + 2].copy_from_slice(&0u16.to_le_bytes());
    out[off + 2..off + 4].copy_from_slice(&1u16.to_le_bytes());
    out[off + 4..off + 6].copy_from_slice(&0u16.to_le_bytes());
    Some(end)
}

/// Build an association request: capability info, a listen interval, then the
/// SSID and supported-rates elements. `bssid` is the AP being joined.
pub fn assoc_request(
    out: &mut [u8],
    src: MacAddr,
    bssid: MacAddr,
    ssid: &[u8],
    rates: &[u8],
    cap: u16,
    seq: u16,
) -> Option<usize> {
    let fc = frame_control(TYPE_MGMT, SUBTYPE_ASSOC_REQ);
    let off = write_header(out, fc, bssid, src, bssid, seq)?;
    let body = off + 4;
    if body > out.len() {
        return None;
    }
    out[off..off + 2].copy_from_slice(&cap.to_le_bytes());
    out[off + 2..off + 4].copy_from_slice(&10u16.to_le_bytes());
    let mut cur = body;
    cur = put_ie(out, cur, IE_SSID, ssid)?;
    cur = put_ie(out, cur, IE_SUPPORTED_RATES, rates)?;
    Some(cur)
}
