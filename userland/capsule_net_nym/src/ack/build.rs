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

use super::plaintext::ack_plaintext;
use super::types::{FRAG_ID_BYTES, PADDED_ADDRESS_BYTES};
use crate::crypto::random::fill_random;
use crate::crypto::types::CryptoError;
use crate::sphinx::constants::{ACK_PAYLOAD_SIZE, DESTINATION_ADDRESS_LENGTH, PACKET_VERSION};
use crate::sphinx::node::{Destination, Node};
use crate::sphinx::packet::build_packet_sized;

/// Build the acknowledgement that rides along with a message.
///
/// A recipient does not answer with this; it forwards it, and the mixnet
/// carries it back to us. That is what tells us a packet arrived without the
/// recipient learning who to tell. It is not optional: a recipient reads the
/// payload as an ack followed by the message, so a packet without one does
/// not parse and is dropped before the message is ever seen.
///
/// The bytes are the address the ack enters the network at, padded to a fixed
/// width, then the ack packet itself.
pub fn build_surb_ack(
    route: &[Node],
    delays: &[[u8; 8]],
    our_identity: &[u8; DESTINATION_ADDRESS_LENGTH],
    ack_key: &[u8; 16],
    frag_id: [u8; FRAG_ID_BYTES],
) -> Result<Vec<u8>, CryptoError> {
    let Some(first_hop) = route.first() else {
        return Err(CryptoError::Kdf);
    };

    let plaintext = ack_plaintext(ack_key, frag_id)?;

    let mut secret = [0u8; 32];
    fill_random(&mut secret)?;
    let destination = Destination { address: *our_identity, identifier: [0u8; 16] };
    let packet = build_packet_sized(
        &secret,
        route,
        &destination,
        delays,
        PACKET_VERSION,
        &plaintext,
        ACK_PAYLOAD_SIZE,
    )?;

    let mut out = Vec::with_capacity(PADDED_ADDRESS_BYTES);
    out.extend_from_slice(&first_hop.address[..PADDED_ADDRESS_BYTES.min(first_hop.address.len())]);
    out.resize(PADDED_ADDRESS_BYTES, 0);
    out.extend_from_slice(&packet.to_bytes_sized(ACK_PAYLOAD_SIZE).ok_or(CryptoError::Kdf)?);
    Ok(out)
}
