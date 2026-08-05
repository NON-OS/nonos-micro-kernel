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

/// A route ending at the gateway a named recipient holds its session with.
///
/// A recipient's address names three things, and the last of them is not
/// decoration: only the gateway it registered with can hand a packet to it.
/// Ending the route at any other gateway delivers to a node that has never
/// heard of the recipient, and the failure is silent in a way that looks like
/// success from here. That node still unwraps the last layer, still lifts the
/// acknowledgement out of the payload and sends it back, and then drops the
/// message it cannot deliver. Every packet is answered and nothing arrives.
pub fn route_to(seed: &[u8; 32], gateway_identity: &[u8; 32]) -> Option<Vec<SphinxNode>> {
    let Some(gateway) = topology::node_by_identity(gateway_identity) else {
        crate::trace::say(b"route to exit: its gateway is not in the directory yet");
        return None;
    };
    if gateway.role != Role::EntryGateway && gateway.role != Role::ExitGateway {
        crate::trace::say(b"route to exit: its gateway is not a gateway in the directory");
        return None;
    }
    let Some(hops) = topology::route(seed).ok() else {
        crate::trace::say(b"route to exit: no mix route available");
        return None;
    };
    let mut route = Vec::with_capacity(hops.len());
    // The mix layers are drawn like any other route; only the egress is
    // pinned, because that is the one hop the recipient is reachable through.
    for hop in hops.iter().take(hops.len() - 1) {
        route.push(convert(hop));
    }
    route.push(convert(&gateway));
    Some(route)
}

fn convert(node: &TopoNode) -> SphinxNode {
    SphinxNode { address: routing_address(node.ip, node.port), pub_key: node.packet_key }
}
