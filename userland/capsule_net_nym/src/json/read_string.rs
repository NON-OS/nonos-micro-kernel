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

use alloc::string::String;

/// Read a JSON string starting at `at`, undoing the basic escapes.
pub fn read_string(text: &[u8], mut at: usize) -> Option<String> {
    while at < text.len() && (text[at] as char).is_whitespace() {
        at += 1;
    }
    if text.get(at) != Some(&b'"') {
        return None;
    }
    at += 1;
    let mut out = String::new();
    while at < text.len() {
        match text[at] {
            b'"' => return Some(out),
            b'\\' => {
                at += 1;
                match text.get(at)? {
                    b'n' => out.push('\n'),
                    b'r' => out.push('\r'),
                    b't' => out.push('\t'),
                    other => out.push(*other as char),
                }
            }
            other => out.push(other as char),
        }
        at += 1;
    }
    None
}
