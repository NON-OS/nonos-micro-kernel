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

//! The station data-path engine shared by every WiFi driver. A driver holds one
//! `LinkStation`, feeds it its association facts, and then converts between
//! ethernet and 802.11 with two calls; its `LinkPort` implementation becomes a
//! thin shim over its own rings.
//!
//! The engine keeps the difference between a hardware-CCMP chip (the radio
//! encrypts from its key cache, so the station only frames) and a software-CCMP
//! chip (the station runs AES-CCM itself) in one place, so no driver picks the
//! wrong path and double-encrypts. It owns the transmit sequence number and, for
//! software CCMP, the packet-number counter, so those are never duplicated per
//! driver either.

use alloc::vec::Vec;

use crate::dot11::data::{build_data, parse_data, protect, unprotect};
use crate::dot11::header::MacAddr;

/// How the link protects data frames.
pub enum Ccmp {
    /// The chip encrypts and decrypts in hardware from its key cache. The
    /// station frames plaintext and lets the radio insert the CCMP header and
    /// manage packet numbers.
    Hardware,
    /// The station runs AES-CCM in software with the pairwise temporal key for
    /// both directions.
    Software { tk: [u8; 16] },
    /// The station encrypts transmit frames in software but leaves receive
    /// decryption to the radio. For a chip whose hardware transmit encryption
    /// does not work but whose receive decryption does.
    SoftwareTxHwRx { tk: [u8; 16] },
}

/// The per-station data path: association facts plus the transmit counters.
pub struct LinkStation {
    our_mac: MacAddr,
    bssid: MacAddr,
    ccmp: Ccmp,
    pn: u64,
    seq: u16,
    associated: bool,
}

impl LinkStation {
    /// A fresh, unassociated station with the given MAC. It refuses to transmit
    /// until `associate` records a BSSID and CCMP mode.
    pub fn new(our_mac: MacAddr) -> Self {
        Self { our_mac, bssid: [0u8; 6], ccmp: Ccmp::Hardware, pn: 0, seq: 0, associated: false }
    }

    /// Record association to `bssid` with the given CCMP mode, resetting the
    /// sequence and packet-number counters for the new session.
    pub fn associate(&mut self, bssid: MacAddr, ccmp: Ccmp) {
        self.bssid = bssid;
        self.ccmp = ccmp;
        self.pn = 0;
        self.seq = 0;
        self.associated = true;
    }

    /// Drop the association; the station stops framing until it associates again.
    pub fn deassociate(&mut self) {
        self.associated = false;
    }

    /// Whether the station is associated and carrying data.
    pub fn is_associated(&self) -> bool {
        self.associated
    }

    /// The station MAC address.
    pub fn mac(&self) -> MacAddr {
        self.our_mac
    }

    /// Frame an ethernet packet for transmission to the AP. For a hardware-CCMP
    /// chip this is the plaintext MPDU the radio will encrypt; for software CCMP
    /// it is the fully encrypted frame. Advances the sequence number, and for
    /// software CCMP the packet number. Returns `None` if the station is not
    /// associated or the ethernet frame is malformed.
    pub fn tx_frame(&mut self, eth: &[u8]) -> Option<Vec<u8>> {
        if !self.associated {
            return None;
        }
        let seq = self.seq;
        let mpdu = build_data(eth, self.our_mac, self.bssid, seq)?;
        // Only commit the counter once framing has succeeded.
        self.seq = self.seq.wrapping_add(1) & 0x0FFF;
        match &self.ccmp {
            Ccmp::Hardware => {
                // Pure-hardware CCMP hands the radio a plaintext frame; the engine
                // inserts the IV and ciphertext and sets the Protected bit itself.
                Some(mpdu)
            }
            Ccmp::Software { tk } | Ccmp::SoftwareTxHwRx { tk } => {
                // CCMP packet numbers start at 1; pre-increment from zero.
                let pn = self.pn.wrapping_add(1);
                let frame = protect(&mpdu, pn, tk)?;
                self.pn = pn;
                Some(frame)
            }
        }
    }

    /// Recover the ethernet frame from a received 802.11 data MPDU. For a
    /// hardware-CCMP chip the MPDU is already decrypted, so it is only parsed;
    /// for software CCMP it is decrypted first. Returns `None` if the frame is
    /// malformed, the MIC fails, or the body is not LLC/SNAP encapsulated.
    pub fn rx_frame(&self, mpdu: &[u8]) -> Option<Vec<u8>> {
        match &self.ccmp {
            Ccmp::Hardware | Ccmp::SoftwareTxHwRx { .. } => parse_data(mpdu),
            Ccmp::Software { tk } => parse_data(&unprotect(mpdu, tk)?),
        }
    }
}
