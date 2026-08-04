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

use super::base58::decode32;
use super::field::{ipv4_field, nested_u64, string_field, u64_field};
use crate::topology::{Node, Role};

/// Per-hop delay asked of a mix. Reordering is where the anonymity comes
/// from, not speed.
const HOP_DELAY_MS: u16 = 50;

/// Where a node takes packets when it does not say otherwise.
const DEFAULT_MIX_PORT: u64 = 1789;

/// Where a gateway holds client sessions when it does not say otherwise.
const DEFAULT_WS_PORT: u64 = 9000;

/// Read one node out of an API object.
///
/// A record missing any field a route needs is dropped rather than filled in
/// with a default: a hop addressed wrongly cannot decrypt what is sealed for
/// it, so a guess costs a dead route rather than a slower one.
pub fn parse_node(obj: &[u8], role: Role) -> Option<Node> {
    // A node advertises every address it answers on. The first is the one the
    // route is built against; a hop is reached at one address or not at all.
    let ip = ipv4_field(&string_field(obj, "ip_addresses")?)?;
    let port = u64_field(obj, "mix_port").unwrap_or(DEFAULT_MIX_PORT) as u16;
    // A gateway takes packets on one port and client sessions on another, so
    // the routing address cannot double as somewhere to dial. Mixes hold no
    // sessions and advertise nothing here.
    let ws_port = match role {
        Role::Mix => 0,
        _ => nested_u64(obj, "entry", "ws_port").unwrap_or(DEFAULT_WS_PORT) as u16,
    };
    // The layer sits inside the role object, and a key lookup deliberately
    // does not reach into one, so it is read from that span.
    let layer = match role {
        Role::Mix => nested_u64(obj, "role", "layer")? as u8,
        _ => 0,
    };
    if role == Role::Mix && !(1..=3).contains(&layer) {
        return None;
    }
    let identity = decode32(string_field(obj, "ed25519_identity_pubkey")?.as_slice())?;
    let packet_key = decode32(string_field(obj, "x25519_sphinx_pubkey")?.as_slice())?;
    Some(Node { role, layer, delay_ms: HOP_DELAY_MS, ip, port, ws_port, identity, packet_key })
}
