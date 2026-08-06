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

use super::fragment::{Fragment, MAX_FRAGMENTS};
use super::plaintext::PLAINTEXT_PER_PACKET;
use super::repliable::{pad_to_packets, repliable_data};
use super::types::SENDER_TAG_SIZE;

/// A message split into the pieces each packet will carry.
pub struct Prepared {
    pub fragments: Vec<Vec<u8>>,
}

/// Turn a request into the fragments that will be sent.
///
/// The reply blocks travel with the request rather than being asked for
/// later, because the recipient has no way to ask: it never learns who sent
/// this. Padding happens before splitting so that the number of packets, and
/// each one's width, says nothing beyond how much was sent.
pub fn prepare(
    sender_tag: &[u8; SENDER_TAG_SIZE],
    reply_surbs: &[Vec<u8>],
    request: &[u8],
    set_id: i32,
) -> Option<Prepared> {
    prepare_built(repliable_data(sender_tag, reply_surbs, request), set_id)
}

/// Split a message that is already built.
///
/// Not every message carries a request. A top up of reply blocks is a message
/// in its own right and travels the same way, so the splitting is shared and
/// only the building differs.
pub fn prepare_built(message: Vec<u8>, set_id: i32) -> Option<Prepared> {
    let padded = pad_to_packets(message, PLAINTEXT_PER_PACKET)?;

    let total = padded.len() / PLAINTEXT_PER_PACKET;
    if total == 0 || total > MAX_FRAGMENTS as usize {
        return None;
    }

    let mut fragments = Vec::with_capacity(total);
    for (index, chunk) in padded.chunks(PLAINTEXT_PER_PACKET).enumerate() {
        // Positions are counted from one. Zero is what an absent field reads
        // as, and the far end refuses it rather than guess.
        let header = Fragment { set_id, total: total as u8, current: (index + 1) as u8 };
        fragments.push(header.into_bytes(chunk));
    }
    Some(Prepared { fragments })
}
