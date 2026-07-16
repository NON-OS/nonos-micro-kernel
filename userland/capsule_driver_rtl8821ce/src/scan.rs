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

//! The networks a scan finds. The channel hop and the ring reads live in the
//! serving stage (they need the radio); this is the pure collection they feed:
//! deduplicate by access point, drop hidden networks, cap the list, and encode in
//! the format the settings panel already parses: a count, then per network a
//! signal byte, a flags byte (bit 0 secured), the SSID length and the SSID.
//! Signal strength is left at zero until the per-frame PHY status is decoded; the
//! network list is the win. Kept free of hardware so it is checked on the host.

/// The most networks a scan reports.
pub const MAX_RESULTS: usize = 16;
const SSID_MAX: usize = 32;
const FLAG_SECURED: u8 = 0x01;

#[derive(Clone, Copy)]
struct Entry {
    ssid: [u8; SSID_MAX],
    ssid_len: u8,
    bssid: [u8; 6],
    secured: bool,
}

/// The networks a scan found, deduplicated by access point.
pub struct ScanResults {
    entries: [Entry; MAX_RESULTS],
    count: usize,
}

impl Default for ScanResults {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanResults {
    pub fn new() -> Self {
        let empty = Entry { ssid: [0; SSID_MAX], ssid_len: 0, bssid: [0; 6], secured: false };
        Self { entries: [empty; MAX_RESULTS], count: 0 }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Record one beacon. Hidden networks (an empty SSID) and access points
    /// already seen are ignored, and the list is capped at `MAX_RESULTS`.
    pub fn add(&mut self, bssid: [u8; 6], ssid: &[u8], secured: bool) {
        if ssid.is_empty() || ssid.len() > SSID_MAX || self.count >= MAX_RESULTS {
            return;
        }
        if self.entries[..self.count].iter().any(|e| e.bssid == bssid) {
            return;
        }
        let mut e = Entry { ssid: [0; SSID_MAX], ssid_len: ssid.len() as u8, bssid, secured };
        e.ssid[..ssid.len()].copy_from_slice(ssid);
        self.entries[self.count] = e;
        self.count += 1;
    }

    /// Encode for the settings panel: `[count]` then per network `[signal][flags]
    /// [ssid_len][ssid]`. Returns the number of bytes written; a network that
    /// would overflow `out` is dropped rather than truncated.
    pub fn encode(&self, out: &mut [u8]) -> usize {
        if out.is_empty() {
            return 0;
        }
        let mut o = 1;
        let mut written = 0u8;
        for e in &self.entries[..self.count] {
            let n = e.ssid_len as usize;
            if o + 3 + n > out.len() {
                break;
            }
            out[o] = 0; // signal: PHY-status RSSI not decoded yet
            out[o + 1] = if e.secured { FLAG_SECURED } else { 0 };
            out[o + 2] = e.ssid_len;
            out[o + 3..o + 3 + n].copy_from_slice(&e.ssid[..n]);
            o += 3 + n;
            written += 1;
        }
        out[0] = written;
        o
    }
}
