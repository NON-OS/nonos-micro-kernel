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

//! One list at a time, in the order a route needs them.

use alloc::vec::Vec;

use super::budget_roles::{ENTRY_BUDGET, EXIT_BUDGET, MIX_BUDGET};
use super::live::{fetch_exits, fetch_gateways, fetch_mixnodes, layers_present};
use super::step::{Step, PARTIAL};
use crate::topology::{self, Node};
/// The mix layers, which are what a route is built from.
pub(super) fn first(tcp_port: u32) -> Step {
    match fetch_mixnodes(tcp_port) {
        Ok(mut nodes) if layers_present(&nodes) => {
            nodes.truncate(MIX_BUDGET);
            *PARTIAL.lock() = Some((nodes, 1));
            Step::Progressed
        }
        // A list missing a layer cannot carry a route, and keeping it would
        // let a later step install something unroutable.
        Ok(_) => Step::Failed(13),
        Err(code) => Step::Failed(code),
    }
}

/// The gateways a session is held with.
pub(super) fn gateways(tcp_port: u32, mut nodes: Vec<Node>) -> Step {
    if let Ok(mut found) = fetch_gateways(tcp_port) {
        found.truncate(ENTRY_BUDGET);
        nodes.append(&mut found);
    }
    *PARTIAL.lock() = Some((nodes, 2));
    Step::Progressed
}

/// The gateways a packet leaves by, and then the install.
///
/// A missing exit list is not fatal on its own, but it is what an exit is
/// found behind, so a directory without one cannot open a connection.
pub(super) fn exits(tcp_port: u32, mut nodes: Vec<Node>) -> Step {
    if let Ok(mut found) = fetch_exits(tcp_port) {
        found.truncate(EXIT_BUDGET);
        nodes.append(&mut found);
    }
    let count = nodes.len();
    match topology::install_fetched(nodes) {
        Ok(()) => Step::Done(count),
        Err(_) => Step::Failed(20),
    }
}
