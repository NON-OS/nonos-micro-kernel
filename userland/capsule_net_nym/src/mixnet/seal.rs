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

use super::delays::hop_delays;
use super::mix_packet::frame_mix_packet;
use super::route_to::route_to;
use crate::crypto::random::fill_random;
use crate::sphinx::constants::{DESTINATION_ADDRESS_LENGTH, PACKET_VERSION};
use crate::sphinx::node::Destination;
use crate::sphinx::packet::build_packet;

/// Seal one payload into a packet and frame it for the gateway.
///
/// The route is drawn per packet rather than per message. Two packets of the
/// same message then share no path, which is what stops a mix that sees both
/// from grouping them.
pub fn seal_one(
    destination: &[u8; DESTINATION_ADDRESS_LENGTH],
    gateway_identity: &[u8; 32],
    payload: &[u8],
) -> Option<Vec<u8>> {
    let mut seed = [0u8; 32];
    fill_random(&mut seed).ok()?;
    let route = route_to(&seed, gateway_identity)?;
    let delays = hop_delays(&seed)?;
    let mut secret = [0u8; 32];
    fill_random(&mut secret).ok()?;
    // The identifier is sent as zeros, which is what a reference client puts
    // there: this network does not use the field.
    let dest = Destination { address: *destination, identifier: [0u8; 16] };
    let first_hop = route[0].address;
    let packet = build_packet(&secret, &route, &dest, &delays, PACKET_VERSION, payload).ok()?;
    Some(frame_mix_packet(&first_hop, &packet.to_bytes()?))
}

/// Delays for a route of `hops` length, drawn fresh so an acknowledgement
/// cannot be timed against the packet it belongs to.
pub fn hop_delays_for(hops: usize) -> Option<Vec<[u8; 8]>> {
    let mut seed = [0u8; 32];
    fill_random(&mut seed).ok()?;
    let mut delays = hop_delays(&seed)?;
    delays.truncate(hops);
    if delays.len() != hops {
        return None;
    }
    Some(delays)
}
