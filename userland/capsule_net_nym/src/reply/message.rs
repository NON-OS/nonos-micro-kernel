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

//! The message a reassembled reply turns out to be.

use super::types::{RECIPIENT_BYTES, TAG_REPLY_DATA, TAG_REPLY_SURB_REQUEST, TYPE_REPLY};
use crate::message::unpad;

/// What a reply was.
pub enum Reply<'a> {
    /// Bytes for whoever asked.
    Data(&'a [u8]),
    /// The far end is low on reply blocks and cannot answer until it has
    /// more. It names where to send them and how many it wants.
    SurbRequest { recipient: [u8; RECIPIENT_BYTES], amount: u32 },
}

/// Strip the message layer and say what was actually sent.
///
/// A reply names its own type and content, and both are checked rather than
/// skipped: a message that says it is something else has either been
/// tampered with or belongs to a protocol this does not speak, and reading
/// past the tags would hand its body on as though it were ours.
pub fn reply_message(reassembled: &[u8]) -> Option<Reply<'_>> {
    let message = unpad(reassembled)?;
    if message.len() < 2 || message[0] != TYPE_REPLY {
        return None;
    }
    match message[1] {
        TAG_REPLY_DATA => Some(Reply::Data(&message[2..])),
        TAG_REPLY_SURB_REQUEST => {
            let body = &message[2..];
            if body.len() < RECIPIENT_BYTES + 4 {
                return None;
            }
            let mut recipient = [0u8; RECIPIENT_BYTES];
            recipient.copy_from_slice(&body[..RECIPIENT_BYTES]);
            let amount = u32::from_be_bytes([
                body[RECIPIENT_BYTES],
                body[RECIPIENT_BYTES + 1],
                body[RECIPIENT_BYTES + 2],
                body[RECIPIENT_BYTES + 3],
            ]);
            Some(Reply::SurbRequest { recipient, amount })
        }
        _ => None,
    }
}

/// The data of a reply, for callers that only handle that.
pub fn reply_body(reassembled: &[u8]) -> Option<&[u8]> {
    match reply_message(reassembled)? {
        Reply::Data(body) => Some(body),
        Reply::SurbRequest { .. } => None,
    }
}
