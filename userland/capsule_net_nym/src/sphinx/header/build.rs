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

use super::built::BuiltHeader;
use super::derive_keys::derive_hop_keys;
use super::final_block::build_final_block;
use super::types::SphinxHeader;
use super::wrap_hops::wrap_forward_hops;
use crate::crypto::ecdh::x25519_public;
use crate::crypto::types::CryptoError;
use crate::sphinx::constants::VERSION_LENGTH;
use crate::sphinx::filler::build_filler;
use crate::sphinx::mac::compute_mac;
use crate::sphinx::node::{Destination, Node};

/// Build a header for `route`, ending at `destination`.
pub fn build_header(
    initial_secret: &[u8; 32],
    route: &[Node],
    destination: &Destination,
    delays: &[[u8; 8]],
    version: [u8; VERSION_LENGTH],
) -> Result<BuiltHeader, CryptoError> {
    if route.is_empty() || route.len() != delays.len() {
        return Err(CryptoError::Kdf);
    }
    let secrets = derive_hop_keys(initial_secret, route)?;
    let last = &secrets[route.len() - 1];
    let filler = build_filler(&secrets[..route.len() - 1]).ok_or(CryptoError::Kdf)?;
    let routing = build_final_block(last, destination, route.len(), version, &filler)?;
    let mac = compute_mac(&last.integrity_mac_key(), &routing)?;
    let (routing, mac) = wrap_forward_hops(route, &secrets, delays, version, (routing, mac))?;

    let mut ephemeral_pubkey = [0u8; 32];
    x25519_public(initial_secret, &mut ephemeral_pubkey)?;
    Ok(BuiltHeader {
        header: SphinxHeader { ephemeral_pubkey, integrity_mac: mac, routing_info: routing },
        payload_keys: secrets.iter().map(|s| s.legacy_payload_key()).collect(),
    })
}
