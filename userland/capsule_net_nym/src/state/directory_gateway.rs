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

use super::gateway::{Gateway, Transport};
use crate::topology::{self, Role};

/// An entry gateway from the live directory, by position.
///
/// Preferred over the compiled list because that list ages: operators come
/// and go, and a client that only ever dials five addresses is both easier
/// to starve and easier to recognise. The compiled list stays as the way in
/// when no directory has arrived yet.
///
/// A gateway is dialled on the port it advertises for client sessions, not
/// the one it takes packets on. Those differ, and using the routing port
/// reaches a listener that will not speak this protocol.
pub fn directory_gateway(index: usize) -> Option<Gateway> {
    let nodes = topology::snapshot().ok()?;
    let mut seen = 0usize;
    for node in nodes.iter().filter(|n| n.role == Role::EntryGateway) {
        if seen == index {
            return Some(Gateway {
                ip: node.ip,
                port: node.ws_port,
                stream: 0,
                transport: Transport::WebSocket,
                identity: node.identity,
                shared_key: [0u8; 32],
            });
        }
        seen += 1;
    }
    None
}

/// How many entry gateways the directory offers.
pub fn directory_gateway_count() -> usize {
    match topology::snapshot() {
        Ok(nodes) => nodes.iter().filter(|n| n.role == Role::EntryGateway).count(),
        Err(_) => 0,
    }
}

/// How many exit gateways the directory offers. A directory with gateways but
/// no exit can build a route home yet has nowhere to leave the mixnet, so the
/// sync is not done until this is non-zero.
pub fn directory_exit_count() -> usize {
    match topology::snapshot() {
        Ok(nodes) => nodes.iter().filter(|n| n.role == Role::ExitGateway).count(),
        Err(_) => 0,
    }
}
