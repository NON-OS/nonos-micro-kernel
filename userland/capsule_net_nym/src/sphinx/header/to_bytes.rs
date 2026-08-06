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
use crate::sphinx::constants::{HEADER_INTEGRITY_MAC_SIZE, HEADER_SIZE};

impl SphinxHeader {
    pub fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut out = [0u8; HEADER_SIZE];
        out[..32].copy_from_slice(&self.ephemeral_pubkey);
        out[32..32 + HEADER_INTEGRITY_MAC_SIZE].copy_from_slice(&self.integrity_mac);
        out[32 + HEADER_INTEGRITY_MAC_SIZE..].copy_from_slice(&self.routing_info);
        out
    }
}
