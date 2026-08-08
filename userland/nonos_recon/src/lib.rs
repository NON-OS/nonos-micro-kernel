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

//! The scan engine behind the `recon` tool.
//!
//! Parsing the target and port list, deciding what to probe, and turning the
//! per-port outcomes into a report all live here and are pure, so they are
//! host-tested against the awkward inputs a scanner sees. The actual connect is
//! a `Probe` the caller supplies: the capsule drives real TCP over the net
//! stack, a test drives a table of canned outcomes, and the orchestration is
//! identical either way.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod ports;
mod report;
mod scan;
mod target;

pub use ports::{parse_ports, PortError, MAX_PORTS};
pub use report::{format_report, PortState, ScanRow};
pub use scan::{scan, Probe};
pub use target::{parse_target, Target, TargetError};

#[cfg(test)]
mod tests;
