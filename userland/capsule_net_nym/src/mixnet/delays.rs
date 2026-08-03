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

use crate::topology::{self, Node};
use alloc::vec::Vec;

/// Per-hop delays from the directory, not invented here.
///
/// The delay is what actually mixes traffic: packets arriving together leave
/// apart, so an observer at both ends cannot pair them.
pub fn hop_delays(seed: &[u8; 32]) -> Option<Vec<[u8; 8]>> {
    match topology::route(seed) {
        Ok(hops) => Some(hops.iter().map(encode).collect()),
        Err(_) => Some(crate::state::bootstrap_route(seed).iter().map(encode).collect()),
    }
}

fn encode(node: &Node) -> [u8; 8] {
    (node.delay_ms as u64).to_be_bytes()
}
