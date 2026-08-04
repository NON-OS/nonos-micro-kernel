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

use super::types::{ReplySurb, SURB_KEY_BYTES};
use crate::sphinx::constants::PAYLOAD_KEY_SIZE;

/// Serialize a reply block the way the far end reads it.
///
/// Layout is the encryption key, the header, the address of the hop a reply
/// enters the network at, then one payload key per hop. The key leads because
/// it is ours rather than part of the route, and the far end strips it before
/// handing the rest to the packet layer.
pub fn surb_bytes(surb: &ReplySurb) -> Vec<u8> {
    let mut out = Vec::with_capacity(
        SURB_KEY_BYTES
            + surb.header.len()
            + surb.first_hop_address.len()
            + surb.payload_keys.len() * PAYLOAD_KEY_SIZE,
    );
    out.extend_from_slice(&surb.key);
    out.extend_from_slice(&surb.header);
    out.extend_from_slice(&surb.first_hop_address);
    for hop_key in &surb.payload_keys {
        out.extend_from_slice(hop_key);
    }
    out
}
