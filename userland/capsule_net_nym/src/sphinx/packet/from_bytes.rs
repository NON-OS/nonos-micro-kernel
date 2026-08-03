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
use crate::sphinx::constants::{HEADER_SIZE, REGULAR_PACKET_SIZE};
use crate::sphinx::header::SphinxHeader;
use alloc::vec::Vec;

impl SphinxPacket {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != REGULAR_PACKET_SIZE {
            return None;
        }
        let header = SphinxHeader::from_bytes(&bytes[..HEADER_SIZE])?;
        let mut payload = Vec::with_capacity(bytes.len() - HEADER_SIZE);
        payload.extend_from_slice(&bytes[HEADER_SIZE..]);
        Some(Self { header, payload })
    }
}
