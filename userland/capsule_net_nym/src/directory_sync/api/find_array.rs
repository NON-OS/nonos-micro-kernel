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

//! Where a named array begins.

/// The offset of the `[` opened by `key`.
///
/// Only an array counts. The same name holding an object is a different
/// field, and reading that as a list would find nothing in it.
pub(super) fn find_array(text: &[u8], key: &str) -> Option<usize> {
    let needle = key.as_bytes();
    let mut i = 0usize;
    while i + needle.len() + 2 < text.len() {
        if text[i] == b'"'
            && text[i + 1..].starts_with(needle)
            && text[i + 1 + needle.len()] == b'"'
        {
            let mut at = i + needle.len() + 2;
            while at < text.len() && (text[at] as char).is_whitespace() {
                at += 1;
            }
            if text.get(at) != Some(&b':') {
                i += 1;
                continue;
            }
            at += 1;
            while at < text.len() && (text[at] as char).is_whitespace() {
                at += 1;
            }
            return if text.get(at) == Some(&b'[') { Some(at) } else { None };
        }
        i += 1;
    }
    None
}
