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

use super::clock;
use super::directory::{DirectoryMeta, ParsedDirectory, Provenance};
use super::store;
use super::types::{Node, TopologyError};

/// How long a fetched list is used before it is asked for again. Long enough
/// that a sync is not constant traffic, short enough that a node which left
/// the network stops being routed through within the hour.
const VALID_MS: u64 = 60 * 60 * 1000;

/// Install a node list fetched from the API.
///
/// This replaces whatever is installed, including the table that shipped in
/// the image, which is the point: the compiled list is a starting position,
/// not the network. It carries an expiry because a fetched view goes stale,
/// unlike the image table whose age is bounded by the rollback index.
pub fn install_fetched(nodes: Vec<Node>) -> Result<(), TopologyError> {
    let now = clock::now_ms()?;
    let meta = DirectoryMeta {
        // The epoch advances with the clock so a later fetch is never
        // mistaken for a replay of an earlier one.
        epoch: now,
        not_before_ms: now,
        not_after_ms: now.saturating_add(VALID_MS),
        issuer: [0u8; 32],
        provenance: Provenance::Fetched,
    };
    store::replace(ParsedDirectory { meta, nodes }, now)
}
