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

use crate::crypto::ecdh::x25519_shared;
use crate::crypto::types::CryptoError;
use crate::sphinx::keys::{expand_shared_secret, ExpandedSharedSecret};
use crate::sphinx::node::Node;
use alloc::vec::Vec;

/// One shared secret per hop.
///
/// Each hop's secret is the node's key put through this packet's ephemeral
/// scalar and then through every preceding hop's blinding factor. That chain
/// is what lets hop n derive its own key from the single group element in the
/// header while learning nothing about the hops after it.
pub fn derive_hop_keys(
    initial_secret: &[u8; 32],
    route: &[Node],
) -> Result<Vec<ExpandedSharedSecret>, CryptoError> {
    let mut expanded = Vec::with_capacity(route.len());
    let mut blindings: Vec<[u8; 32]> = Vec::with_capacity(route.len());
    for node in route {
        let mut acc = node.pub_key;
        let mut scalar = *initial_secret;
        for step in 0..=blindings.len() {
            if step > 0 {
                scalar = blindings[step - 1];
            }
            let mut next = [0u8; 32];
            x25519_shared(&scalar, &acc, &mut next)?;
            acc = next;
        }
        let secret = expand_shared_secret(&acc)?;
        blindings.push(secret.blinding_factor());
        expanded.push(secret);
    }
    Ok(expanded)
}
