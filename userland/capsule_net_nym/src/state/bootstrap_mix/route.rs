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

use super::table::{BOOTSTRAP_MIXNODES, PER_LAYER};
use crate::topology::{Node, Role};

/// One mix from each layer, picked by `seed`.
///
/// The table is grouped by layer, so the candidate block for layer n starts at
/// n * PER_LAYER and the seed only has to choose within it. Delay stays zero
/// here: without a directory there is no published delay to honour, and
/// inventing one would shape traffic in a way the network did not ask for.
pub fn bootstrap_route(seed: &[u8; 32]) -> [Node; 3] {
    let mut hops = [EMPTY; 3];
    for (index, hop) in hops.iter_mut().enumerate() {
        let base = index * PER_LAYER;
        let pick = base + (seed[index] as usize % PER_LAYER);
        let (ip, port, packet_key) = BOOTSTRAP_MIXNODES[pick];
        *hop = Node {
            role: Role::Mix,
            layer: (index + 1) as u8,
            delay_ms: 0,
            ip,
            port,
            ws_port: 0,
            identity: [0u8; 32],
            packet_key,
        };
    }
    hops
}

const EMPTY: Node = Node {
    role: Role::Mix,
    layer: 0,
    delay_ms: 0,
    ip: [0u8; 4],
    port: 0,
    ws_port: 0,
    identity: [0u8; 32],
    packet_key: [0u8; 32],
};
