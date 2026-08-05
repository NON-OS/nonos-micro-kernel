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

use crate::sphinx::constants::{PAYLOAD_OVERHEAD_SIZE, SECURITY_PARAMETER};

/// Recover the message from a fully unwrapped payload.
///
/// The leading zeros are checked rather than skipped. They are the only
/// evidence the last layer came off under the right key, so a payload that
/// does not carry them is noise that happened to be the right length, and
/// reading a message out of it would mean reading whatever an attacker sent.
pub fn unpad_payload(padded: &[u8]) -> Option<&[u8]> {
    if padded.len() < PAYLOAD_OVERHEAD_SIZE {
        return None;
    }
    if padded[..SECURITY_PARAMETER].iter().any(|b| *b != 0) {
        return None;
    }
    let body = &padded[SECURITY_PARAMETER..];
    let end = body.iter().rposition(|b| *b == 0x01)?;
    if body[end + 1..].iter().any(|b| *b != 0) {
        return None;
    }
    Some(&body[..end])
}
