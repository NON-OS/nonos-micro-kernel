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

use crate::sphinx::constants::{ENCRYPTED_ROUTING_INFO_SIZE, HEADER_INTEGRITY_MAC_SIZE};

/// The 348-byte header: the sender's ephemeral key, the MAC over the routing
/// info, and the routing info itself.
pub struct SphinxHeader {
    pub ephemeral_pubkey: [u8; 32],
    pub integrity_mac: [u8; HEADER_INTEGRITY_MAC_SIZE],
    pub routing_info: [u8; ENCRYPTED_ROUTING_INFO_SIZE],
}
