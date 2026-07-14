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

//! The WPA2-PSK 4-way handshake, supplicant side. This is chip-independent:
//! it consumes only the proven key-derivation (`wpa::ptk`), the EAPOL codec
//! (`eapol`), and the AES key-unwrap (`ccmp::keywrap`), so any WiFi driver
//! (Intel, Realtek) drives the same state machine. Given the PMK and the two
//! MAC addresses, `step` is fed each EAPOL-Key frame the AP sends and returns
//! the frame to transmit back, deriving the pairwise key on message 1 and
//! installing the group key on message 3.

mod state;
pub mod step;

pub use state::{State, Supplicant};
