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

use crate::crypto::types::CryptoError;
use crate::sphinx::constants::{
    ENCRYPTED_ROUTING_INFO_SIZE, FORWARD_HOP_FLAG, HEADER_INTEGRITY_MAC_SIZE, VERSION_LENGTH,
};
use crate::sphinx::keys::ExpandedSharedSecret;
use crate::sphinx::mac::compute_mac;
use crate::sphinx::node::Node;
use crate::sphinx::routing::{encrypt_routing_info, truncate, RoutingInformation};

type Sealed = ([u8; ENCRYPTED_ROUTING_INFO_SIZE], [u8; HEADER_INTEGRITY_MAC_SIZE]);

/// Wrap each earlier hop around the block built so far, working backwards.
/// Hop n is told the address of hop n+1, which is why the first hop's own
/// address never appears: the sender already knows it.
pub fn wrap_forward_hops(
    route: &[Node],
    secrets: &[ExpandedSharedSecret],
    delays: &[[u8; 8]],
    version: [u8; VERSION_LENGTH],
    start: Sealed,
) -> Result<Sealed, CryptoError> {
    let (mut routing, mut mac) = start;
    for hop in (0..route.len() - 1).rev() {
        let info = RoutingInformation {
            flag: FORWARD_HOP_FLAG,
            version,
            node_address: route[hop + 1].address,
            delay: delays[hop],
            next_mac: mac,
            next_routing: truncate(&routing),
        };
        routing = encrypt_routing_info(&info, &secrets[hop].stream_cipher_key())
            .ok_or(CryptoError::Kdf)?;
        mac = compute_mac(&secrets[hop].integrity_mac_key(), &routing)?;
    }
    Ok((routing, mac))
}
