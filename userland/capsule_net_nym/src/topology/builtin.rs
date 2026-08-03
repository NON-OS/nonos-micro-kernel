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

/// Delay each hop is asked to hold a packet for, in milliseconds.
///
/// The mixnet's guarantee comes from packets being reordered against each
/// other, not from arriving quickly, so this buys shuffling at every layer.
const HOP_DELAY_MS: u16 = 50;

/// Publish the mixnodes compiled into this image as the route directory.
///
/// No signature is made or checked. These nodes arrived inside a kernel that
/// the bootloader measured, verified against two signatures, and matched to
/// its STARK enrollment before jumping to it. Signing the list again would
/// mean minting a key whose theft would redirect every route, which is a
/// worse position than the one it claims to improve.
///
/// A fetched directory replaces this and does have to prove itself, since
/// those bytes carry none of the above.
pub fn install() -> Result<(), TopologyError> {
    let now = super::clock::now_ms()?;
    let nodes: Vec<Node> = BOOTSTRAP_MIXNODES
        .iter()
        .enumerate()
        .map(|(i, (ip, port, packet_key))| Node {
            role: Role::Mix,
            // Three per layer, laid out layer one first, so the index names
            // the layer a node forwards from.
            layer: (i / PER_LAYER) as u8 + 1,
            delay_ms: HOP_DELAY_MS,
            ip: *ip,
            port: *port,
            // A mix hop is authenticated by its packet key when the header is
            // sealed for it. The identity key names gateways, and no route
            // built from this table consults one.
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
