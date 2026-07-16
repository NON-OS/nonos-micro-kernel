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

//! The wire format the panel exchanges with the WiFi service: scan results in,
//! a connect request out. Scan results are `[count][entry...]` where each entry
//! is `[signal][flags][ssid_len][ssid]` and flags bit 0 marks a secured
//! network. A connect request is `[ssid_len][ssid][passphrase]`. Pure parsing
//! and encoding over untrusted bytes, bounds-checked, no panics.

#[cfg(test)]
use alloc::vec::Vec;

use super::network::{ScanNetwork, SSID_MAX};

const FLAG_SECURED: u8 = 1 << 0;

/// Parse a scan-result buffer into networks, invoking `push` for each. Stops at
/// the first malformed entry rather than trusting a bad length. Returns how many
/// networks were read.
pub fn parse_scan(buf: &[u8], mut push: impl FnMut(ScanNetwork)) -> usize {
    if buf.is_empty() {
        return 0;
    }
    let count = buf[0] as usize;
    let mut off = 1;
    let mut got = 0;
    while got < count {
        if off + 3 > buf.len() {
            break;
        }
        let signal = buf[off];
        let flags = buf[off + 1];
        let ssid_len = buf[off + 2] as usize;
        let start = off + 3;
        let end = match start.checked_add(ssid_len) {
            Some(e) if e <= buf.len() && ssid_len <= SSID_MAX => e,
            _ => break,
        };
        push(ScanNetwork::new(&buf[start..end], signal, flags & FLAG_SECURED != 0));
        off = end;
        got += 1;
    }
    got
}

/// Encode a connect request for `ssid` with `passphrase` (empty for an open
/// network). Wired when joining lands; exercised now in the panel proofs.
#[cfg(test)]
pub fn encode_connect(ssid: &[u8], passphrase: &[u8]) -> Vec<u8> {
    let len = ssid.len().min(SSID_MAX);
    let mut out = Vec::with_capacity(1 + len + passphrase.len());
    out.push(len as u8);
    out.extend_from_slice(&ssid[..len]);
    out.extend_from_slice(passphrase);
    out
}
