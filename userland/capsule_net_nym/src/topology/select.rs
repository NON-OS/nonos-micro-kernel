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

use super::store;
use super::types::{Node, Role, RouteError};

/// One hop from each mix layer, then the gateway the packet leaves by.
///
/// Five nodes carry the packet, but only four of them are hops the header
/// holds a layer for. The entry gateway is the fifth: we hand it the packet
/// over the websocket we are already holding, and it forwards to the first
/// hop the mix packet names. Listing it as that first hop asked it to forward
/// the packet to itself.
pub fn route(seed: &[u8; 32]) -> Result<[Node; 4], RouteError> {
    let nodes = store::snapshot()?;
    Ok([
        pick(&nodes, Role::Mix, 1, seed, 1)?,
        pick(&nodes, Role::Mix, 2, seed, 2)?,
        pick(&nodes, Role::Mix, 3, seed, 3)?,
        pick(&nodes, Role::ExitGateway, 0, seed, 4)?,
    ])
}

fn pick(
    nodes: &[Node],
    role: Role,
    layer: u8,
    seed: &[u8; 32],
    salt: u8,
) -> Result<Node, RouteError> {
    let matches = matching(nodes, role, layer);
    if matches.is_empty() {
        return Err(RouteError::MissingHop);
    }
    let idx = (seed[salt as usize] as usize) % matches.len();
    Ok(matches[idx])
}

fn matching(nodes: &[Node], role: Role, layer: u8) -> Vec<Node> {
    nodes
        .iter()
        .copied()
        .filter(|n| n.role == role && (role != Role::Mix || n.layer == layer))
        .collect()
}
