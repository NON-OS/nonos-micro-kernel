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

//! Finding an exit that answers.

use crate::directory_sync::{fetch_exit, ExitAddress};
use crate::setup;
use crate::topology::{self, Role};
use crate::trace;

/// How many exits to ask before giving up.
///
/// A node is asked directly for the requester it runs, and a node that is
/// listed is not necessarily one that answers: it may be busy, behind a
/// filter, or not running the interface at all. Stopping at the first
/// silence would make one unlucky node look like a network with no exits.
const CANDIDATES: usize = 6;

/// An exit that answered, starting from `index`.
///
/// Callers pass an index so they can ask for a different one than last time.
/// The walk starts there and moves on, so a caller that wants variety gets it
/// without having to know which nodes are reachable.
pub fn find_exit(index: usize) -> Option<ExitAddress> {
    let nodes = topology::snapshot().ok()?;
    let exits: alloc::vec::Vec<_> = nodes.iter().filter(|n| n.role == Role::ExitGateway).collect();
    if exits.is_empty() {
        trace::say(b"exit lookup: the directory lists none");
        return None;
    }
    trace::say_num(b"exit lookup: candidates", exits.len() as u64);

    let port = setup::tcp_port();
    for step in 0..CANDIDATES.min(exits.len()) {
        let node = exits[(index + step) % exits.len()];
        let Some(found) = fetch_exit(port, node.ip, node.identity) else {
            continue;
        };
        // An exit is only usable if we can address the gateway it sits
        // behind. We hold a slice of the network, so a requester whose
        // gateway is outside it answers here and is then unreachable: the
        // route cannot be built, and the failure lands later and further
        // away than it needs to.
        if topology::node_by_identity(&found.gateway).is_none() {
            trace::say(b"exit lookup: skipped one whose gateway we cannot route to");
            continue;
        }
        trace::say_num(b"exit lookup: answered after tries", step as u64 + 1);
        return Some(found);
    }
    trace::say_num(b"exit lookup: none answered of", CANDIDATES as u64);
    None
}
