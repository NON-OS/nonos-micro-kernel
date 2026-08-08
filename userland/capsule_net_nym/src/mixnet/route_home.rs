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

use super::address::routing_address;
use crate::sphinx::node::Node as SphinxNode;
use crate::topology::{self, Node as TopoNode, Role};

/// A route back to us, ending at the gateway holding our session.
///
/// Replies and acknowledgements both travel this way. The last hop has to be
/// our own gateway, because that is the only node that can hand something to
/// a client it holds a session with; anywhere else the packet arrives with
/// nobody to give it to.
///
/// Returns nothing until the directory knows our gateway. We hold a session
/// with it by identity, but routing to it needs the address and packet key it
/// publishes, and those cannot be guessed.
pub fn route_home(seed: &[u8; 32], gateway_identity: &[u8; 32]) -> Option<Vec<SphinxNode>> {
    let Some(gateway) = topology::node_by_identity(gateway_identity) else {
        crate::trace::say(b"route home: our gateway is not in the directory yet");
        return None;
    };
    if gateway.role != Role::EntryGateway && gateway.role != Role::ExitGateway {
        crate::trace::say(b"route home: our gateway is not a gateway in the directory");
        return None;
    }
    let Some(hops) = topology::route(seed).ok() else {
        crate::trace::say(b"route home: no mix route available");
        return None;
    };
    let mut home = Vec::with_capacity(hops.len());
    // Everything up to the egress is a mix like any other; only the last hop
    // differs, and it is ours rather than the one the forward route ends at.
    for hop in hops.iter().take(hops.len() - 1) {
        home.push(convert(hop));
    }
    home.push(convert(&gateway));
    Some(home)
}

fn convert(node: &TopoNode) -> SphinxNode {
    SphinxNode { address: routing_address(node.ip, node.port), pub_key: node.packet_key }
}
