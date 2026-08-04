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

use super::types::{TAG_REPLY_DATA, TYPE_REPLY};
use crate::message::unpad;

/// Strip the message layer and hand back what was actually sent.
///
/// A reply names its own type and content, and both are checked rather than
/// skipped: a message that says it is something else has either been
/// tampered with or belongs to a protocol this does not speak, and reading
/// past the tags would hand its body on as though it were ours.
pub fn reply_body(reassembled: &[u8]) -> Option<&[u8]> {
    let message = unpad(reassembled)?;
    if message.len() < 2 {
        return None;
    }
    if message[0] != TYPE_REPLY || message[1] != TAG_REPLY_DATA {
        return None;
    }
    Some(&message[2..])
}
