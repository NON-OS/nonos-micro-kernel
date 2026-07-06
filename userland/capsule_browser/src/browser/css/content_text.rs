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

// The text of a content declaration: the first quoted string with CSS escapes
// decoded, notably the hex form icon fonts use ("\e90d"). none, counters,
// attr() and url() yield None and the pseudo box is skipped.
pub(super) fn content_text(value: &str) -> Option<String> {
    let v = value.trim();
    let quote = *v.as_bytes().first()?;
    if quote != b'"' && quote != b'\'' {
        return None;
    }
    let inner = &v[1..v.rfind(quote as char)?.max(1)];
    let mut out = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let mut hex = String::new();
        while hex.len() < 6 {
            match chars.peek() {
                Some(c) if c.is_ascii_hexdigit() => {
                    hex.push(*c);
                    chars.next();
                }
                _ => break,
            }
        }
        if hex.is_empty() {
            // A non-hex escape stands for the character itself.
            if let Some(c) = chars.next() {
                out.push(c);
            }
        } else {
            // One whitespace after a hex escape terminates it and is eaten.
            if chars.peek() == Some(&' ') {
                chars.next();
            }
            if let Some(c) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                out.push(c);
            }
        }
        if out.len() > 256 {
            break;
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}
