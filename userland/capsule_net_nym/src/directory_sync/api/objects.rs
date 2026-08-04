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

use alloc::vec::Vec;

/// Split a JSON array into its top-level objects.
///
/// Brace counting rather than a full parse: the fields a route needs are
/// flat, and a reader that understands less of a stranger's document is a
/// reader with less to get wrong. Strings are tracked so a brace inside one
/// does not shift the depth.
pub fn objects(text: &[u8], cap: usize) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (i, &b) in text.iter().enumerate() {
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
            b'{' => {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            }
            b'}' => {
                if depth == 0 {
                    return out;
                }
                depth -= 1;
                if depth == 0 {
                    out.push(&text[start..=i]);
                    if out.len() == cap {
                        return out;
                    }
                }
            }
            _ => {}
        }
    }
    out
}
