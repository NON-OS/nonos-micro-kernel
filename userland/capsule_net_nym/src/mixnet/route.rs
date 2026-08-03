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

use crate::sphinx::node::Node as SphinxNode;
use crate::topology::{self, Node as TopoNode};
use alloc::vec::Vec;

/// Turn a selected route into what Sphinx needs.
///
/// A mix is addressed by its identity key while packets are sealed to its
/// separate packet key. Conflating the two is the mistake this exists to
/// make impossible.
pub fn sphinx_route(seed: &[u8; 32]) -> Option<Vec<SphinxNode>> {
    let hops = topology::route(seed).ok()?;
    Some(hops.iter().map(convert).collect())
}

fn convert(node: &TopoNode) -> SphinxNode {
    SphinxNode { address: node.identity, pub_key: node.packet_key }
}
