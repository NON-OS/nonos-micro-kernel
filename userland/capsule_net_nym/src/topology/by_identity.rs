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

use super::store;
use super::types::Node;

/// The directory's record for a node we know only by identity.
///
/// A gateway we hold a session with is known by the key it authenticated
/// with, but routing a packet to it needs the address and packet key the
/// directory publishes. Those are not interchangeable, so one has to be
/// looked up from the other.
pub fn node_by_identity(identity: &[u8; 32]) -> Option<Node> {
    let nodes = store::snapshot().ok()?;
    nodes.into_iter().find(|n| &n.identity == identity)
}
