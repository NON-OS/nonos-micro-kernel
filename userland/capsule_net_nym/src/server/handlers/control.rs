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

use crate::json::{find_key, read_string};
use crate::trace;

/// Longest slice of a gateway message worth putting in a line.
const QUOTE_MAX: usize = 56;

/// The gateway names this when it priced a packet it had no credit for.
const OUT_OF_BANDWIDTH: &[u8] = b"out_of_bandwidth";

/// Report what a gateway said in the clear, and say whether it was a refusal
/// for want of allowance.
///
/// Control messages are JSON, not blobs, and they are where a gateway
/// explains itself: a refused packet, an exhausted allowance, a session it no
/// longer recognises. Running them through the blob parser turned every one
/// of them into the same authentication failure, so the explanation for a
/// send going nowhere was thrown away each time it arrived.
pub fn note_control(text: &[u8]) -> bool {
    if contains(text, OUT_OF_BANDWIDTH) {
        trace::say(b"gateway refused the packet, no allowance left");
        return true;
    }
    if let Some(message) = field(text, "message") {
        quote(b"gateway says", message.as_bytes());
    } else if let Some(kind) = field(text, "type") {
        quote(b"gateway control", kind.as_bytes());
    } else {
        quote(b"gateway control, unread", text);
    }
    false
}

fn field(text: &[u8], key: &str) -> Option<alloc::string::String> {
    read_string(text, find_key(text, key)?)
}

fn contains(text: &[u8], needle: &[u8]) -> bool {
    text.windows(needle.len()).any(|w| w == needle)
}

fn quote(stage: &[u8], body: &[u8]) {
    // A cut mid-character still reads as a message, and the alternative is
    // dropping the line that says what went wrong.
    trace::say_text(stage, &body[..body.len().min(QUOTE_MAX)]);
}
