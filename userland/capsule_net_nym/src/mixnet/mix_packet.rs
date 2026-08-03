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

/// A forward packet destined for the mixnet rather than a reply.
const PACKET_TYPE_MIX: u8 = 0;

/// Bytes of an IPv4 routing address that are actually used: version, port,
/// four octets. The Sphinx header pads this out; the gateway framing does not.
const ROUTING_ADDRESS_V4_LEN: usize = 7;

/// Frame a Sphinx packet for the gateway.
///
/// The gateway is handed a mix packet, not a bare Sphinx packet: it has to
/// know which mix to forward to, and the first hop is deliberately absent from
/// the header because the sender already knows it. Layout is the packet type,
/// then the next hop unpadded, then the packet.
pub fn frame_mix_packet(first_hop_address: &[u8; 32], packet: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + ROUTING_ADDRESS_V4_LEN + packet.len());
    out.push(PACKET_TYPE_MIX);
    out.extend_from_slice(&first_hop_address[..ROUTING_ADDRESS_V4_LEN]);
    out.extend_from_slice(packet);
    out
}
