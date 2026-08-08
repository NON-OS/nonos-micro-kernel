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

//! How much of a packet is left for a message once its overheads are taken.

use super::fragment::UNLINKED_HEADER_LEN;
use crate::ack::PADDED_ADDRESS_BYTES;
use crate::sphinx::constants::{ACK_PAYLOAD_SIZE, HEADER_SIZE, REGULAR_PACKET_PLAINTEXT};

/// Bytes an acknowledgement takes inside a payload: the address it enters the
/// network at, padded, then the ack packet itself.
pub const ACK_OVERHEAD: usize = PADDED_ADDRESS_BYTES + HEADER_SIZE + ACK_PAYLOAD_SIZE;

/// Bytes the per packet key agreement takes: the public half travels so the
/// recipient can repeat it.
pub const KEY_AGREEMENT_OVERHEAD: usize = 32;

/// What is left for a fragment, header included.
pub const FRAGMENT_PER_PACKET: usize =
    REGULAR_PACKET_PLAINTEXT - ACK_OVERHEAD - KEY_AGREEMENT_OVERHEAD;

/// What is left for the message itself, once the fragment header is taken.
pub const PLAINTEXT_PER_PACKET: usize = FRAGMENT_PER_PACKET - UNLINKED_HEADER_LEN;

// A packet that cannot hold its own overheads would make every message
// unsendable, and the sizes it is derived from are not obviously large
// enough to rule that out by eye.
const _: () = assert!(PLAINTEXT_PER_PACKET > 0);
