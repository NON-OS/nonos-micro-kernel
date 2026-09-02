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

/// How much of `b` is usable as a hostname, counting from the front.
///
/// The bytes come off an IPC reply, so they are sanitized before they are
/// drawn: the count stops at the first byte a hostname cannot contain, which
/// also trims the NUL padding a fixed-width store sends back.
pub fn hostname_len(b: &[u8]) -> usize {
    let mut n = 0;
    while n < b.len() && usable(b[n]) {
        n += 1;
    }
    n
}

fn usable(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' || c == b'.'
}
