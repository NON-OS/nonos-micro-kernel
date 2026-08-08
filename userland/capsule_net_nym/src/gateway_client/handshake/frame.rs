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

use crate::json::{find_key, read_bytes, read_string, tagged_bytes_request};
use alloc::string::String;
use alloc::vec::Vec;

pub const INIT_TAG: &str = "registerHandshakeInitRequest";
pub const PAYLOAD_TAG: &str = "handshakePayload";

pub fn encode(tag: &str, version: u64, data: &[u8]) -> String {
    tagged_bytes_request(tag, version, data)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HandshakeFrameError {
    Malformed,
    Refused,
}

/// Pull the payload out of a handshake frame. A `handshakeError` frame carries
/// a message rather than data, so it is reported as a refusal.
pub fn decode(text: &[u8]) -> Result<Vec<u8>, HandshakeFrameError> {
    let tag = find_key(text, "type")
        .and_then(|at| read_string(text, at))
        .ok_or(HandshakeFrameError::Malformed)?;
    if tag == "handshakeError" {
        return Err(HandshakeFrameError::Refused);
    }
    find_key(text, "data").and_then(|at| read_bytes(text, at)).ok_or(HandshakeFrameError::Malformed)
}
