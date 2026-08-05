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
use crate::sphinx::constants::{HEADER_SIZE, REGULAR_PAYLOAD_SIZE};
use alloc::vec::Vec;

impl SphinxPacket {
    /// Serialize a packet whose payload is the usual width.
    pub fn to_bytes(&self) -> Option<Vec<u8>> {
        self.to_bytes_sized(REGULAR_PAYLOAD_SIZE)
    }

    /// Serialize a packet whose payload is `width` bytes.
    ///
    /// The width is checked rather than assumed, because it is what tells a
    /// hop which kind of packet it is holding, and a packet built to one
    /// width and written at another would be read as neither.
    ///
    /// Acknowledgements are the reason this takes a width at all: they travel
    /// in a narrower packet than messages, so a serializer that only knew the
    /// message width refused every one of them and no message could carry the
    /// ack it has to include.
    pub fn to_bytes_sized(&self, width: usize) -> Option<Vec<u8>> {
        if self.payload.len() != width {
            return None;
        }
        let mut out = Vec::with_capacity(HEADER_SIZE + width);
        out.extend_from_slice(&self.header.to_bytes());
        out.extend_from_slice(&self.payload);
        Some(out)
    }
}
