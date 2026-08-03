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

use super::delays::hop_delays;
use super::route::sphinx_route;
use crate::crypto::random::fill_random;
use crate::sphinx::constants::DESTINATION_ADDRESS_LENGTH;
use crate::sphinx::node::Destination;
use crate::sphinx::packet::build_packet;
use alloc::vec::Vec;

const VERSION: [u8; 3] = [1, 0, 0];

/// Build one Sphinx packet for `payload`.
///
/// The ephemeral scalar is fresh per packet: reusing one would give every
/// packet the same per-hop keys and let a single mix link them.
pub fn encode_sphinx(
    destination: &[u8; DESTINATION_ADDRESS_LENGTH],
    identifier: &[u8; 16],
    payload: &[u8],
) -> Option<Vec<u8>> {
    let mut seed = [0u8; 32];
    fill_random(&mut seed).ok()?;
    let route = sphinx_route(&seed)?;
    let delays = hop_delays(&seed)?;
    let mut secret = [0u8; 32];
    fill_random(&mut secret).ok()?;
    let dest = Destination { address: *destination, identifier: *identifier };
    build_packet(&secret, &route, &dest, &delays, VERSION, payload).ok()?.to_bytes()
}
