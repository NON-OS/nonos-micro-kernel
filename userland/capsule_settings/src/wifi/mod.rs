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
//! (`network`, `wire`, `scan_client`). The scan client also joins a network and
//! reads the driver's data-path counts (`datapath`); `lease` reads the address
//! net_core bound. The panel shows the networks, the connection, and its address.

mod adapters;
mod datapath;
mod interface;
mod lease;
mod net_status;
mod network;
mod scan_client;
mod wire;

pub use adapters::scan_adapters;
pub use datapath::{driver_datapath, DataPath};
pub use interface::WifiInterface;
pub use lease::{Lease, NetStatus};
pub use net_status::net_status;
pub use network::ScanNetwork;
pub use scan_client::{
    connect_network, driver_stage, scan_networks, ConnectResult, DriverStage, ScanOutcome,
    ScanStats,
};
