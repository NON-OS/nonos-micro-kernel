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
//! Reading what a receive-pack made of a push.

extern crate alloc;

use crate::transport::TransportError;
use crate::wire::{read_pkt, Pkt};

/// Check the report a receiver sends after a push.
///
/// The report is the only place a push learns whether it worked. A connection
/// that closes cleanly having refused every ref looks exactly like a success
/// at the socket, so this insists on seeing `unpack ok` and then a line
/// starting `ok ` for the ref, and treats anything else as a refusal.
pub(super) fn accepted(response: &[u8], name: &str) -> Result<(), TransportError> {
    let mut at = 0usize;
    let mut unpacked = false;
    let mut moved = false;
    while at < response.len() {
        let (pkt, used) = read_pkt(&response[at..]).map_err(|_| TransportError::Malformed)?;
        at += used;
        let Pkt::Data(line) = pkt else {
            continue;
        };
        // The side-band wraps each line in a one-byte stream number.
        let line = match line.first() {
            Some(1) => &line[1..],
            _ => line,
        };
        if line.starts_with(b"unpack ok") {
            unpacked = true;
        }
        if line.starts_with(b"ok ") && line[3..].starts_with(name.as_bytes()) {
            moved = true;
        }
    }
    if unpacked && moved {
        Ok(())
    } else {
        Err(TransportError::Refused)
    }
}
