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

use super::types::SphinxPacket;
use crate::sphinx::constants::{HEADER_SIZE, REGULAR_PACKET_SIZE, REGULAR_PAYLOAD_SIZE};
use alloc::vec::Vec;

impl SphinxPacket {
    pub fn to_bytes(&self) -> Option<Vec<u8>> {
        if self.payload.len() != REGULAR_PAYLOAD_SIZE {
            return None;
        }
        let mut out = Vec::with_capacity(REGULAR_PACKET_SIZE);
        out.extend_from_slice(&self.header.to_bytes());
        out.extend_from_slice(&self.payload);
        debug_assert_eq!(out.len(), HEADER_SIZE + REGULAR_PAYLOAD_SIZE);
        Some(out)
    }
}
