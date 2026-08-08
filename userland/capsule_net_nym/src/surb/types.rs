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

use crate::sphinx::constants::{HEADER_SIZE, NODE_ADDRESS_LENGTH, PAYLOAD_KEY_SIZE};

/// Bytes of the key a reply is sealed with before its route is applied. This
/// is an AES128 counter mode key, so it is half the width of the keys used
/// elsewhere in the packet.
pub const SURB_KEY_BYTES: usize = 16;

/// A route home, handed to someone who must not learn where home is.
pub struct ReplySurb {
    pub key: [u8; SURB_KEY_BYTES],
    pub header: [u8; HEADER_SIZE],
    pub first_hop_address: [u8; NODE_ADDRESS_LENGTH],
    pub payload_keys: Vec<[u8; PAYLOAD_KEY_SIZE]>,
}
