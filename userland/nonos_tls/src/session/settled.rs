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
//! Whether the server's flight has stopped growing.

/// True once an application-data record appears in the flight.
///
/// Records are typed in their first byte and carry a big-endian length, so
/// walking them is exact rather than a guess about how much has arrived. A
/// type of 23 is application data, which only follows a finished handshake.
pub(super) fn settled(bytes: &[u8]) -> bool {
    let mut at = 0usize;
    while at + 5 <= bytes.len() {
        let len = u16::from_be_bytes([bytes[at + 3], bytes[at + 4]]) as usize;
        let end = at + 5 + len;
        if end > bytes.len() {
            return false;
        }
        if bytes[at] == 23 {
            return true;
        }
        at = end;
    }
    false
}
