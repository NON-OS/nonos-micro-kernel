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

//! The network-interface adapter that binds this WiFi driver to net_core. Once
//! the station is associated, net_core drives it with the same link protocol it
//! uses for a wired NIC, so DHCP, DNS, TCP and everything above them flow over
//! WiFi unchanged. This module owns the protocol side; a `LinkPort`
//! implementation supplies the association state, the CCMP data path and the
//! firmware rings.

pub mod serve;
pub mod wire;

pub use serve::{serve, LinkPort, MAX_RESPONSE};
