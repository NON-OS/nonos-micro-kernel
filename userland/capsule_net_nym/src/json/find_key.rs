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

/// Byte offset just past `"key":` at the top level of `text`.
///
/// Depth-aware so a matching key inside a nested object is not mistaken for
/// the one being looked for, and quote-aware so a key name appearing inside a
/// string value does not match either.
pub fn find_key(text: &[u8], key: &str) -> Option<usize> {
    let needle = key.as_bytes();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0usize;
    while i < text.len() {
        let b = text[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' => {
                if depth == 1 && text[i + 1..].starts_with(needle) {
                    let after = i + 1 + needle.len();
                    if text.get(after) == Some(&b'"') {
                        let mut j = after + 1;
                        while j < text.len() && (text[j] as char).is_whitespace() {
                            j += 1;
                        }
                        if text.get(j) == Some(&b':') {
                            return Some(j + 1);
                        }
                    }
                }
                in_string = true;
            }
            b'{' | b'[' => depth += 1,
            b'}' | b']' => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    None
}
