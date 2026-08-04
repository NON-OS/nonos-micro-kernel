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

use super::directory::{DirectoryMeta, ParsedDirectory, Provenance};
use super::store;
use super::types::{Node, Role, TopologyError};
use crate::state::{BOOTSTRAP_MIXNODES, PER_LAYER};

/// Per-hop delay. Reordering is where the anonymity comes from, not speed.
const HOP_DELAY_MS: u16 = 50;

/// Publish the mixnodes compiled into this image as the route directory.
///
/// Nothing is signed here. Signing a list that already ships inside an
/// attested kernel would only mint a key whose theft redirects every route. A
/// fetched directory replaces this and still has to prove itself.
pub fn install() -> Result<(), TopologyError> {
    let now = super::clock::now_ms()?;
    let nodes: Vec<Node> = BOOTSTRAP_MIXNODES
        .iter()
        .enumerate()
        .map(|(i, (ip, port, packet_key))| Node {
            role: Role::Mix,
            // Table is laid out layer one first, three per layer.
            layer: (i / PER_LAYER) as u8 + 1,
            delay_ms: HOP_DELAY_MS,
            ip: *ip,
            port: *port,
            // Mixes hold no client sessions, so there is nothing to dial.
            ws_port: 0,
            // Identity names gateways. A mix hop is authenticated by its
            // packet key when the header is sealed for it.
            identity: [0u8; 32],
            packet_key: *packet_key,
        })
        .collect();

    let meta = DirectoryMeta {
        epoch: 0,
        not_before_ms: 0,
        not_after_ms: u64::MAX,
        issuer: [0u8; 32],
        provenance: Provenance::Image,
    };
    store::replace(ParsedDirectory { meta, nodes }, now)
}
