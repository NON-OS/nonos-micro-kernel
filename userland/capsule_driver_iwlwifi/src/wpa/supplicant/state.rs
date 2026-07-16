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

//! Supplicant state: the identities and nonces fixed at the start of a
//! handshake, and the keys derived as it proceeds.

/// Where the handshake stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// Waiting for message 1 (the AP's ANonce).
    Start,
    /// Message 1 seen, PTK derived, message 2 sent; awaiting message 3.
    PtkDerived,
    /// Message 3 verified, group key installed, message 4 sent. Connected.
    Connected,
    /// A frame failed verification (bad MIC or wrong ANonce); the handshake
    /// is abandoned rather than trusting forged key material.
    Failed,
}

#[derive(Clone, Copy)]
pub struct Supplicant {
    pub(super) state: State,
    /// Pairwise master key from passphrase + SSID.
    pub(super) pmk: [u8; 32],
    /// Authenticator (AP) MAC.
    pub(super) aa: [u8; 6],
    /// Supplicant (our) MAC.
    pub(super) spa: [u8; 6],
    /// Our nonce, chosen once per handshake.
    pub(super) snonce: [u8; 32],
    /// AP nonce, learned from message 1.
    pub(super) anonce: [u8; 32],
    /// Pairwise transient key (KCK||KEK||TK), derived on message 1.
    pub(super) ptk: [u8; 48],
    /// Group temporal key, unwrapped from message 3.
    pub(super) gtk: [u8; 32],
    pub(super) gtk_len: usize,
}

impl Supplicant {
    /// Begin a handshake. `snonce` must be freshly random per session; it is
    /// supplied by the caller so this stays pure and host-testable.
    pub fn new(pmk: [u8; 32], aa: [u8; 6], spa: [u8; 6], snonce: [u8; 32]) -> Self {
        Self {
            state: State::Start,
            pmk,
            aa,
            spa,
            snonce,
            anonce: [0u8; 32],
            ptk: [0u8; 48],
            gtk: [0u8; 32],
            gtk_len: 0,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    /// The pairwise temporal key (CCMP encryption key), valid once Connected.
    pub fn tk(&self) -> &[u8] {
        &self.ptk[32..48]
    }

    /// The group temporal key, valid once Connected.
    pub fn gtk(&self) -> &[u8] {
        &self.gtk[..self.gtk_len]
    }

    // KCK: signs EAPOL MICs. KEK: unwraps the group key. Both are slices of the
    // PTK, named here so the handshake code reads like the standard.
    pub(super) fn kck(&self) -> &[u8] {
        &self.ptk[0..16]
    }

    pub(super) fn kek(&self) -> [u8; 16] {
        let mut k = [0u8; 16];
        k.copy_from_slice(&self.ptk[16..32]);
        k
    }
}
