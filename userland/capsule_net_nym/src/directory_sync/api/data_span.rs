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

//! Finding the array of nodes inside the document that carries it.
//!
//! The answer is not a bare list. The nodes sit under a key, beside a status
//! block and a pagination block, so splitting the document at its top level
//! yields one object, the document itself, and no nodes at all.

use super::find_array::find_array;

/// The array named by `key`, without its brackets.
///
/// Returns nothing when the key is absent or does not open an array, so a
/// caller can fall back to reading the whole document as the list. That is
/// what a bare array response looks like, and both shapes are worth reading.
pub fn array_span<'a>(text: &'a [u8], key: &str) -> Option<&'a [u8]> {
    let at = find_array(text, key)?;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for (i, &b) in text[at..].iter().enumerate() {
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&text[at + 1..at + i]);
                }
            }
            _ => {}
        }
    }
    None
}
