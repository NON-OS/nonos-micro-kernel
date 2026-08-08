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

//! Driving the probe across the port list.

extern crate alloc;

use alloc::vec::Vec;

use crate::report::{PortState, ScanRow};
use crate::target::Target;

/// A single TCP connect attempt. The capsule implements this over the net
/// stack; a test implements it with canned outcomes. It reports the state and
/// must not panic, so a transport error surfaces as `Filtered` rather than
/// aborting the scan.
pub trait Probe {
    fn probe(&mut self, target: &Target, port: u16) -> PortState;
}

/// Probe every port in order and collect a row per port. The list is expected
/// sorted and deduplicated, as `parse_ports` returns it, so the report follows
/// port order and no port is probed twice.
pub fn scan(target: &Target, ports: &[u16], probe: &mut dyn Probe) -> Vec<ScanRow> {
    let mut rows = Vec::with_capacity(ports.len());
    for &port in ports {
        let state = probe.probe(target, port);
        rows.push(ScanRow { port, state });
    }
    rows
}
