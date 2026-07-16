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

//! The WiFi settings panel. The active tab lists the wireless adapters the
//! broker discovered (`adapters`, `interface`) and the networks a scan found
//! (`network`, `wire`, `scan_client`). The scan client sends the driver service
//! its scan request and parses the reply; the panel shows the networks or a
//! status line. The connect state machine stays proven-but-deferred until
//! joining is wired.

mod adapters;
mod interface;
mod network;
mod scan_client;
mod wire;

pub use adapters::scan_adapters;
pub use interface::WifiInterface;
pub use network::ScanNetwork;
pub use scan_client::{
    connect_network, driver_stage, scan_networks, ConnectResult, DriverStage, ScanOutcome,
    ScanStats,
};
