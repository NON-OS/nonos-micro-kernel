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


//! Guest path to store key. The store has no cwd and no dot-dot, so both
//! are resolved here.

use alloc::vec::Vec;

/// Dot-dot past the root stops at the root, as Linux does.
pub fn absolute(cwd: &[u8], path: &[u8]) -> Vec<u8> {
    let mut joined: Vec<u8> = Vec::new();
    if path.first() != Some(&b'/') {
        joined.extend_from_slice(cwd);
        joined.push(b'/');
    }
    joined.extend_from_slice(path);

    let mut parts: Vec<&[u8]> = Vec::new();
    for part in joined.split(|b| *b == b'/') {
        match part {
            b"" | b"." => {}
            b".." => {
                parts.pop();
            }
            name => parts.push(name),
        }
    }

    let mut out: Vec<u8> = Vec::new();
    for part in parts {
        out.push(b'/');
        out.extend_from_slice(part);
    }
    if out.is_empty() {
        out.push(b'/');
    }
    out
}
