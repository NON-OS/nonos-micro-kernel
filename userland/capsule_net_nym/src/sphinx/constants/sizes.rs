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

use super::fields::*;

/// What a forwarding hop is told: where to send it, the flag, the delay, the
/// version.
pub const NODE_META_INFO_SIZE: usize =
    NODE_ADDRESS_LENGTH + FLAG_LENGTH + DELAY_LENGTH + VERSION_LENGTH;

/// The final hop gets a destination and identifier instead of an address and
/// delay, so its meta info is a different width.
pub const FINAL_NODE_META_INFO_LENGTH: usize =
    DESTINATION_ADDRESS_LENGTH + IDENTIFIER_LENGTH + FLAG_LENGTH + VERSION_LENGTH;

pub const ENCRYPTED_ROUTING_INFO_SIZE: usize =
    (NODE_META_INFO_SIZE + HEADER_INTEGRITY_MAC_SIZE) * MAX_PATH_LENGTH;

/// A hop's worth of routing info is stripped at each mix, so the keystream
/// covers one more slot than the path length to refill the tail.
pub const STREAM_CIPHER_OUTPUT_LENGTH: usize =
    (NODE_META_INFO_SIZE + HEADER_INTEGRITY_MAC_SIZE) * (MAX_PATH_LENGTH + 1);

pub const HEADER_SIZE: usize = 32 + HEADER_INTEGRITY_MAC_SIZE + ENCRYPTED_ROUTING_INFO_SIZE;
pub const PAYLOAD_OVERHEAD_SIZE: usize = SECURITY_PARAMETER + 1;
pub const SPHINX_PACKET_OVERHEAD: usize = HEADER_SIZE + PAYLOAD_OVERHEAD_SIZE;

pub const REGULAR_PACKET_PLAINTEXT: usize = 2 * 1024;
pub const REGULAR_PAYLOAD_SIZE: usize = REGULAR_PACKET_PLAINTEXT + PAYLOAD_OVERHEAD_SIZE;
pub const REGULAR_PACKET_SIZE: usize = HEADER_SIZE + REGULAR_PAYLOAD_SIZE;

pub const EXPANDED_SHARED_SECRET_LENGTH: usize = STREAM_CIPHER_KEY_SIZE
    + INTEGRITY_MAC_KEY_SIZE
    + PAYLOAD_KEY_SIZE
    + BLINDING_FACTOR_SIZE
    + REPLAY_TAG_SIZE;

/// What a hop passes on: the routing info minus the slot it consumed. The
/// stripped slot is refilled from the filler, which is why the header length
/// never changes as a packet travels.
pub const TRUNCATED_ROUTING_INFO_SIZE: usize =
    ENCRYPTED_ROUTING_INFO_SIZE - (NODE_META_INFO_SIZE + HEADER_INTEGRITY_MAC_SIZE);

/// A hop decrypts against its own slot plus one zero-padded slot ahead.
pub const PADDED_ENCRYPTED_ROUTING_INFO_SIZE: usize =
    ENCRYPTED_ROUTING_INFO_SIZE + NODE_META_INFO_SIZE + HEADER_INTEGRITY_MAC_SIZE;

// The predecessor of this module got the 2413 total right and then split it
// 365/2048. 365 is the combined header and payload overhead, so using it as
// the header length steals 17 bytes from the payload: nothing self-tested
// catches it and every mix rejects the packet.
const _: () = assert!(NODE_META_INFO_SIZE == 44);
const _: () = assert!(ENCRYPTED_ROUTING_INFO_SIZE == 300);
const _: () = assert!(HEADER_SIZE == 348);
const _: () = assert!(PAYLOAD_OVERHEAD_SIZE == 17);
const _: () = assert!(SPHINX_PACKET_OVERHEAD == 365);
const _: () = assert!(REGULAR_PAYLOAD_SIZE == 2065);
const _: () = assert!(REGULAR_PACKET_SIZE == 2413);
const _: () = assert!(EXPANDED_SHARED_SECRET_LENGTH == 288);
const _: () = assert!(TRUNCATED_ROUTING_INFO_SIZE == 240);
const _: () = assert!(PADDED_ENCRYPTED_ROUTING_INFO_SIZE == 360);

/// Random prefix an acknowledgement puts before the fragment it names, so no
/// hop can tell two acks from the same client apart.
pub const ACK_IV_SIZE: usize = 16;

/// Bytes naming one fragment: the set id, then the position within it.
pub const FRAG_ID_SIZE: usize = 5;

/// What an acknowledgement carries before padding.
pub const ACK_PLAINTEXT_SIZE: usize = ACK_IV_SIZE + FRAG_ID_SIZE;

/// Payload width of an acknowledgement packet. Acks travel narrower than
/// messages, and the width is how a hop tells which kind it is holding.
pub const ACK_PAYLOAD_SIZE: usize = ACK_PLAINTEXT_SIZE + PAYLOAD_OVERHEAD_SIZE;
