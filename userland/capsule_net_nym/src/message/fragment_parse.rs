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

use super::fragment::{Fragment, SET_ID_MARKER, UNLINKED_HEADER_LEN};

/// Read back a fragment header, returning it with the payload that followed.
pub fn parse(bytes: &[u8]) -> Option<(Fragment, &[u8])> {
    if bytes.len() < UNLINKED_HEADER_LEN {
        return None;
    }
    let raw = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    // A set id arrives with the marker bit set. Without it these are not
    // fragment bytes at all, and reading them as though they were would hand
    // back a set id nobody sent.
    if raw & SET_ID_MARKER == 0 {
        return None;
    }
    // Linked sets carry four more bytes that this does not produce and so
    // cannot read back.
    if bytes[6] != 0 {
        return None;
    }
    let total = bytes[4];
    let current = bytes[5];
    if total == 0 || current == 0 || current > total {
        return None;
    }
    let set_id = raw & !SET_ID_MARKER;
    Some((Fragment { set_id, total, current }, &bytes[UNLINKED_HEADER_LEN..]))
}
