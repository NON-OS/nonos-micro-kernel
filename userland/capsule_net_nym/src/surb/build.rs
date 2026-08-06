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

use super::types::{ReplySurb, SURB_KEY_BYTES};
use crate::crypto::random::fill_random;
use crate::crypto::types::CryptoError;
use crate::sphinx::constants::{DESTINATION_ADDRESS_LENGTH, PACKET_VERSION};
use crate::sphinx::header::build_header;
use crate::sphinx::node::{Destination, Node};

/// Build one reply block for a route that ends at us.
///
/// This is the only way an exit can answer. It never learns where we are, so
/// it cannot address a packet to us; instead it is handed a header we built
/// for a route home, which it puts its reply behind. The route is chosen here
/// and the far end cannot see it or change it.
///
/// The encryption key travels with the block because the reply is sealed with
/// it before the route is applied, and we are the only ones who kept a copy.
pub fn build_surb(
    route: &[Node],
    delays: &[[u8; 8]],
    our_identity: &[u8; DESTINATION_ADDRESS_LENGTH],
) -> Result<ReplySurb, CryptoError> {
    let Some(first_hop) = route.first() else {
        return Err(CryptoError::Kdf);
    };
    let first_hop_address = first_hop.address;

    let mut secret = [0u8; 32];
    fill_random(&mut secret)?;
    // The identifier is unused by this network and is sent as zeros, matching
    // what a reference client puts there.
    let destination = Destination { address: *our_identity, identifier: [0u8; 16] };
    let built = build_header(&secret, route, &destination, delays, PACKET_VERSION)?;

    let mut key = [0u8; SURB_KEY_BYTES];
    fill_random(&mut key)?;

    Ok(ReplySurb {
        key,
        header: built.header.to_bytes(),
        first_hop_address,
        payload_keys: built.payload_keys,
    })
}
