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

//! The 802.11 frame layer. This is the first brick of the association MLME that
//! sits on top of the already-alive firmware: it builds the management frames a
//! scan, an authentication and an association exchange are made of. It holds no
//! device state and touches no register, so it is proven in `iwlwifi_proofs`.
//!
//! Still ahead, on top of this layer: the host-command queue to the firmware,
//! the TX/RX rings, the scan/auth/assoc state machine, and the WPA2 EAPOL
//! four-way handshake with CCMP key install. None of that is wired yet, so the
//! builders here are not called from the driver server path until it is.

pub mod header;
pub mod mgmt;
pub mod data;
pub mod parse;
