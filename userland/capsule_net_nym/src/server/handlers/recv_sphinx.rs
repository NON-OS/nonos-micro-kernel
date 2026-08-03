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

use crate::gateway_client::{is_pushed_message, parse_blob};
use crate::sphinx::payload::unpad_payload;
use crate::state::{gateway_shared_key, TABLE};
use alloc::vec::Vec;

/// Take one encrypted gateway frame and queue what the mixnet delivered.
///
/// The frame is authenticated before anything inside it is read, so a forged
/// push cannot reach a session queue.
pub fn accept_frame(raw: &[u8]) -> bool {
    let Some(key) = gateway_shared_key() else { return false };
    let Some(frame) = parse_blob(raw, &key) else { return false };
    if !is_pushed_message(frame.kind) {
        return false;
    }
    let Some(message) = unpad_payload(&frame.plaintext) else { return false };
    queue(message.to_vec())
}

fn queue(message: Vec<u8>) -> bool {
    TABLE.lock().with_sphinx_session(|s| s.push(message)).is_some()
}
