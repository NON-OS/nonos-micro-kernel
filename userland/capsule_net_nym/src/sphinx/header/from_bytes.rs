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

use super::types::SphinxHeader;
use crate::sphinx::constants::{
    ENCRYPTED_ROUTING_INFO_SIZE, HEADER_INTEGRITY_MAC_SIZE, HEADER_SIZE,
};

impl SphinxHeader {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != HEADER_SIZE {
            return None;
        }
        let mut ephemeral_pubkey = [0u8; 32];
        let mut integrity_mac = [0u8; HEADER_INTEGRITY_MAC_SIZE];
        let mut routing_info = [0u8; ENCRYPTED_ROUTING_INFO_SIZE];
        ephemeral_pubkey.copy_from_slice(&bytes[..32]);
        integrity_mac.copy_from_slice(&bytes[32..32 + HEADER_INTEGRITY_MAC_SIZE]);
        routing_info.copy_from_slice(&bytes[32 + HEADER_INTEGRITY_MAC_SIZE..]);
        Some(Self { ephemeral_pubkey, integrity_mac, routing_info })
    }
}
