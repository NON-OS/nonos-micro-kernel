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

extern crate alloc;

use alloc::vec::Vec;

/// The tunnel is open and whatever follows is stream bytes, of which there
/// may be none.
pub const STREAM_OPEN: u8 = 0;

/// The far end finished. Any bytes that follow are the last of the stream.
pub const STREAM_CLOSED: u8 = 1;

/// What the proxy says back to one request.
///
/// A caller waiting on a mixnet reply asks repeatedly and is usually told
/// there is nothing yet, so "nothing" has to be sayable. The kernel refuses a
/// zero length reply, which left silence as the only way to express it, and
/// silence is not an answer: the caller blocks until its own timeout expires
/// on an answer already known. One leading byte makes the empty answer a real
/// one, and carries whether the tunnel is still open while it is there.
pub struct Reply {
    pub bytes: Vec<u8>,
    pub closed: bool,
}

impl Reply {
    pub fn open(bytes: Vec<u8>) -> Self {
        Self { bytes, closed: false }
    }

    pub fn closed(bytes: Vec<u8>) -> Self {
        Self { bytes, closed: true }
    }

    /// The bytes to put on the wire, marker first.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + self.bytes.len());
        out.push(if self.closed { STREAM_CLOSED } else { STREAM_OPEN });
        out.extend_from_slice(&self.bytes);
        out
    }
}
