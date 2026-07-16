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

//! MLME state: the target network, the BSS chosen from a beacon, and the
//! handshake handed off to the supplicant.

use alloc::vec::Vec;

use crate::wpa::ptk::pmk;
use crate::wpa::supplicant::Supplicant;

/// How far the association has progressed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MlmeState {
    /// Watching beacons for the target SSID.
    Scanning,
    /// A matching BSS was found; open authentication sent.
    Authenticating,
    /// Authenticated; association request sent.
    Associating,
    /// Associated; running the WPA2 four-way handshake.
    FourWay,
    /// Connected: pairwise and group keys installed, link up.
    Connected,
    /// The association failed (auth/assoc refused or the handshake broke).
    Failed,
}

/// What one input produced: the frame to transmit next, if any. The link
/// state is read separately from `Mlme::state`.
pub struct MlmeOutput {
    pub tx: Option<Vec<u8>>,
}

impl MlmeOutput {
    pub(super) fn none() -> Self {
        Self { tx: None }
    }
}

#[derive(Clone, Copy)]
pub struct Mlme {
    pub(super) state: MlmeState,
    pub(super) our_mac: [u8; 6],
    pub(super) ssid: [u8; 32],
    pub(super) ssid_len: usize,
    pub(super) pmk: [u8; 32],
    pub(super) snonce: [u8; 32],
    pub(super) bssid: [u8; 6],
    pub(super) channel: u8,
    pub(super) seq: u16,
    pub(super) supplicant: Option<Supplicant>,
}

impl Mlme {
    /// Begin an association to `ssid` with `passphrase`. `snonce` must be a
    /// fresh random nonce for the eventual handshake; it is passed in so the
    /// state machine stays pure and host-testable.
    pub fn new(our_mac: [u8; 6], ssid: &[u8], passphrase: &[u8], snonce: [u8; 32]) -> Self {
        let mut buf = [0u8; 32];
        let len = ssid.len().min(32);
        buf[..len].copy_from_slice(&ssid[..len]);
        Self {
            state: MlmeState::Scanning,
            our_mac,
            ssid: buf,
            ssid_len: len,
            pmk: pmk(passphrase, &ssid[..len]),
            snonce,
            bssid: [0u8; 6],
            channel: 0,
            seq: 0,
            supplicant: None,
        }
    }

    pub fn state(&self) -> MlmeState {
        self.state
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    /// The pairwise temporal key, valid once Connected.
    pub fn tk(&self) -> Option<&[u8]> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.tk())
    }

    /// The group temporal key, valid once Connected.
    pub fn gtk(&self) -> Option<&[u8]> {
        self.supplicant.as_ref().filter(|_| self.state == MlmeState::Connected).map(|s| s.gtk())
    }

    /// The station MAC and the joined AP's BSSID, needed by the data path to
    /// address encrypted frames. Meaningful once a BSS has been selected.
    pub fn our_mac(&self) -> [u8; 6] {
        self.our_mac
    }

    pub fn bssid(&self) -> [u8; 6] {
        self.bssid
    }

    pub(super) fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }

    // Each transmitted management frame advances the sequence counter.
    pub(super) fn next_seq(&mut self) -> u16 {
        let s = self.seq;
        self.seq = self.seq.wrapping_add(1);
        s
    }
}
