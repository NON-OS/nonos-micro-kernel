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

use alloc::vec::Vec;

use crate::xhci::{port_status, PortSnapshot};

use super::configure_port::configure_port;
use super::constants::{MAX_PORTS, PORTSC_CONNECTED};
use super::types::HidEndpoint;

pub fn enumerate(xhci_port: u32) -> Vec<HidEndpoint> {
    let mut snapshots = [PortSnapshot { port_id: 0, portsc_raw: 0 }; MAX_PORTS];
    let Ok(count) = port_status(xhci_port, &mut snapshots) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for snap in snapshots.iter().take(count).copied() {
        if snap.portsc_raw & PORTSC_CONNECTED != 0 {
            configure_port(xhci_port, snap, &mut out);
        }
    }
    out
}
