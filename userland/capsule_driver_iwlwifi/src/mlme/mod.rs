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

//! The station-side association MLME: the chip-independent brain that takes a
//! network name and passphrase to a connected, keyed link. It consumes the
//! 802.11 frame layer (`dot11`) to scan, open-authenticate and associate, then
//! hands the AP's EAPOL frames to the proven WPA2 `supplicant`. It emits the
//! frames to transmit and never touches hardware, so any driver (Intel,
//! Realtek) drives the same state machine and it is proven on the host.

mod state;
mod step;

pub use state::{Mlme, MlmeOutput, MlmeState};
