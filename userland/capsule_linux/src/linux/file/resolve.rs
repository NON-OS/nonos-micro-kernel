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


//! Turning what a guest asked for into a key the store will accept.
//!
//! A Linux program names files relative to a working directory and uses
//! dot and dot-dot freely. The store has neither, so the path is made
//! absolute and flattened here rather than being handed on and refused.

use alloc::vec::Vec;

/// Join `path` onto `cwd` unless it is already absolute, then remove every
/// dot component and resolve dot-dot against what came before it. A
/// dot-dot that climbs past the root stops at the root, which is what
/// Linux does.
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
